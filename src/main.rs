const H_TOTAL: u32 = 1344;
const V_TOTAL: u32 = 806;
const H_DISPLAY: u32 = 1024;
const V_DISPLAY: u32 = 768;
const VERTICAL_SYNC: f64 = 60.00;
const DOT_CLOCK: u32 = (H_TOTAL as f64 * V_TOTAL as f64 * VERTICAL_SYNC) as u32;
const THREADS: u32 = 4;

use std::sync::{Arc, mpsc};
use std::thread;

use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::event::{Event};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Fullscreen, WindowBuilder};
use winit_input_helper::WinitInputHelper;

mod modulator;

use modulator::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let monitor = event_loop
        .available_monitors()
        //.find(|monitor| monitor.name().unwrap() == "HDMI-0")
        .nth(1) // workaround for my shitty wayland setup because wayland sucks
        .unwrap();
        WindowBuilder::new()
        .with_fullscreen(Some(Fullscreen::Borderless(Some(monitor))))
        .build(&event_loop)?
    };

    let mut pixels = {
        let surface_texture = SurfaceTexture::new(H_DISPLAY, V_DISPLAY, &window);
        Pixels::new(H_DISPLAY, V_DISPLAY, surface_texture)?
    };

    let modulator = AmplitudeModulator {
        carrier: 1700000,
        information: 1000,
    };
    let mut phase_offset = 0;

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            let frame = pixels.get_frame_mut();
            //draw_frame(Box::new(&modulator), phase_offset, frame);
            draw_frame_threaded(Arc::new(modulator.clone()), phase_offset, frame);

            if pixels
            .render()
            .map_err(|e| error!("pixels.render() failed: {}", e))
            .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
            phase_offset = (phase_offset + H_TOTAL * V_TOTAL) % DOT_CLOCK as u32;
        }

        if input.update(&event) {
            window.request_redraw();
        }
    });
}

#[allow(dead_code)]
fn draw_frame(modulator: Box<&dyn Modulator>, phase_offset: u32, frame: &mut [u8]) {
    for pixel in frame.chunks_exact_mut(4).enumerate() {
        let phase = pixel_index_to_phase(pixel.0);
        let phase = Phase { numerator: (phase.numerator + phase_offset) % DOT_CLOCK, denominator: phase.denominator };

        let grayscale = (modulator.sample(phase) * (255.0 / 2.0) + 255.0 / 2.0) as u8;
        pixel.1[0] = grayscale;
        pixel.1[1] = grayscale;
        pixel.1[2] = grayscale;
        pixel.1[3] = 255;
    }
}

fn draw_frame_threaded(modulator: Arc<dyn Modulator + Send + Sync>, phase_offset: u32, frame: &mut [u8]) {
    let (tx, rx) = mpsc::channel();

    let pixels_per_thread = H_DISPLAY * V_DISPLAY / THREADS;
    for i in 0..THREADS {
        let tx = tx.clone();
        let modulator = modulator.clone();
        let chunk = (i * pixels_per_thread)..(i * pixels_per_thread) + pixels_per_thread;

        thread::spawn(move || {
            let mut grayscale_chunk: Vec<u8> = Vec::new();
            for pixel_index in chunk {
                let phase = pixel_index_to_phase(pixel_index as usize);
                let phase = Phase { numerator: (phase.numerator + phase_offset) % DOT_CLOCK, denominator: phase.denominator };

                grayscale_chunk.push((modulator.sample(phase) * (255.0 / 2.0) + 255.0 / 2.0) as u8);
            }
            tx.send((i, grayscale_chunk)).unwrap();
        });
    }
    // If the chunks couldn't be divided evenly, then assign the remaining work to another thread.
    if H_DISPLAY * V_DISPLAY % THREADS != 0 {
        let mut grayscale_chunk: Vec<u8> = Vec::new();
        for pixel_index in (pixels_per_thread * THREADS)..(H_DISPLAY * V_DISPLAY) {
            let phase = pixel_index_to_phase(pixel_index as usize);
            let phase = Phase { numerator: (phase.numerator + phase_offset) % DOT_CLOCK, denominator: phase.denominator };

            grayscale_chunk.push((modulator.sample(phase) * (255.0 / 2.0) + 255.0 / 2.0) as u8);
        }
        tx.send((THREADS, grayscale_chunk)).unwrap();
    }
    drop(tx);

    let mut grayscale_chunks: Vec<(u32, Vec<u8>)> = rx.iter().collect();
    grayscale_chunks.sort_by_key(|thread| thread.0);
    let frame_vec = grayscale_chunks
        .iter()
        .fold(Vec::new(), |acc, chunk| [&acc[..], &chunk.1[..]].concat());

    for pixel in frame_vec.iter().enumerate() {
        frame[pixel.0 * 4] = *pixel.1;
        frame[pixel.0 * 4 + 1] = *pixel.1;
        frame[pixel.0 * 4 + 2] = *pixel.1;
        frame[pixel.0 * 4 + 3] = 255;
    }
}

fn pixel_index_to_phase(pixel_index: usize) -> Phase {
    let numerator = pixel_index as u32
        // Relies on the integer fraction rounding down to skip over each HBlank.
        + (pixel_index as u32 / H_DISPLAY) * (H_TOTAL - H_DISPLAY)
        // Same as above but skips VBlank
        + (pixel_index as u32 / (H_DISPLAY * V_DISPLAY)) * H_TOTAL * (V_TOTAL - V_DISPLAY);
    let denominator = DOT_CLOCK;

    Phase { numerator, denominator }
}
