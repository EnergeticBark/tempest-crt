const H_TOTAL: u32 = 1880;
const V_TOTAL: u32 = 1082;
const H_DISPLAY: u32 = 1400;
const V_DISPLAY: u32 = 1050;
const VERTICAL_SYNC: f64 = 60.00;
const DOT_CLOCK: u32 = (H_TOTAL as f64 * V_TOTAL as f64 * VERTICAL_SYNC) as u32;
const THREADS: u32 = 8;

use std::sync::{mpsc, Arc};
use std::thread;

use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::event::{Event, VirtualKeyCode};
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
            .find(|monitor| monitor.name().unwrap() == "HDMI-1")
            //.nth(1) // workaround for my shitty wayland setup because wayland sucks
            .unwrap();
        WindowBuilder::new()
            .with_fullscreen(Some(Fullscreen::Borderless(Some(monitor))))
            .build(&event_loop)?
    };

    let mut pixels = {
        let surface_texture = SurfaceTexture::new(H_DISPLAY, V_DISPLAY, &window);
        Pixels::new(H_DISPLAY, V_DISPLAY, surface_texture)?
    };

    //let mut wave_freq = 0;
    let pcm_loader: PcmLoader<Signed16Le> = PcmLoader::open("/tmp/virtualdevice", 44100).unwrap();
    //pcm_loader.set_interp(Interpolation::Linear);
    let integrated_loader = PreintegratedLoader::new(pcm_loader);
    let mut carrier = Sine::from_freq(44000000, DOT_CLOCK);
    let mut information = integrated_loader;
    //let mut information = pcm_loader;
    //let mut information = Sine::from_freq(wave_freq, DOT_CLOCK);
    /*let mut modulator = AmplitudeModulator {
        carrier: Arc::from(carrier),
        information: Arc::from(information.samples()),
    };*/
    let mut modulator = FrequencyModulator {
        carrier: Arc::from(carrier),
        information: Arc::from(information.samples()),
    };
    let mut total_index_offset = 0;

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            let frame = pixels.frame_mut();
            //draw_frame(&modulator, total_index_offset, frame);
            draw_frame_threaded(Arc::new(modulator.clone()), frame);

            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }

            carrier.next_frame(H_TOTAL * V_TOTAL);
            information.next_frame().unwrap();
            modulator = FrequencyModulator {
                carrier: Arc::from(carrier),
                information: Arc::from(information.samples()),
            };

            // If the next frame's offset would be more than DOT_CLOCK, then we've been drawing
            // frames for 1 second. Time to load the next second of PCM audio.
            /*if (total_index_offset + H_TOTAL * V_TOTAL) >= DOT_CLOCK {
                /*integrated_loader.next_second().unwrap();
                modulator = FrequencyModulator {
                    carrier: Arc::from(Square::from_freq(29333333)),
                    information: Arc::from(integrated_loader.samples()),
                };*/
                modulator = AmplitudeModulator {
                    carrier: Arc::from(Sine::from_freq(540000, DOT_CLOCK)),
                    information: Arc::from(Square::from_freq(wave_freq, DOT_CLOCK)),
                };
            }*/

            // Add the number of pixels in a total frame to offset the next frame's pixel indices.
            // For example, if there are 100 total pixels in a frame and we're on the 40th
            // frame, then our offset will be 4000 and the next pixel index will be 4001 and so on.
            // If our vertical refresh rate is 60, then after we draw our 60th frame the offset
            // will wrap around back to 0 because of the modulo.
            total_index_offset = (total_index_offset + H_TOTAL * V_TOTAL) % DOT_CLOCK;
        }

        /*if input.key_pressed(VirtualKeyCode::LBracket) {
            wave_freq -= 5;
            information = Sine::from_freq(wave_freq, DOT_CLOCK);
            println!("{wave_freq}");
        } else if input.key_pressed(VirtualKeyCode::RBracket) {
            wave_freq += 5;
            information = Sine::from_freq(wave_freq, DOT_CLOCK);
            println!("{wave_freq}");
        }*/

        if input.update(&event) {
            window.request_redraw();
        }
    });
}

#[allow(dead_code)]
fn draw_frame(modulator: &dyn Signal, frame: &mut [u8]) {
    for pixel in frame.chunks_exact_mut(4).enumerate() {
        let total_index = visible_to_total_index(pixel.0);

        let grayscale = (modulator.sample(total_index) * (255.0 / 2.0) + 255.0 / 2.0).round() as u8;
        pixel.1[0] = grayscale;
        pixel.1[1] = grayscale;
        pixel.1[2] = grayscale;
        pixel.1[3] = 255;
    }
}

fn draw_frame_threaded(modulator: Arc<dyn Signal>, frame: &mut [u8]) {
    let (tx, rx) = mpsc::channel();

    let pixels_per_thread = H_DISPLAY * V_DISPLAY / THREADS;
    for i in 0..THREADS {
        let tx = tx.clone();
        let modulator = modulator.clone();
        let chunk = (i * pixels_per_thread)..(i * pixels_per_thread) + pixels_per_thread;

        thread::spawn(move || {
            let grayscale_chunk = chunk
                .map(|pixel_index| {
                    let total_index = visible_to_total_index(pixel_index as usize);

                    (modulator.sample(total_index) * (255.0 / 2.0) + 255.0 / 2.0).round() as u8
                })
                .collect::<Vec<_>>();

            tx.send((i, grayscale_chunk)).unwrap();
        });
    }
    // If the chunks couldn't be divided evenly, then assign the remaining work to another thread.
    if H_DISPLAY * V_DISPLAY % THREADS != 0 {
        let grayscale_chunk = ((pixels_per_thread * THREADS)..(H_DISPLAY * V_DISPLAY))
            .map(|pixel_index| {
                let total_index = visible_to_total_index(pixel_index as usize);

                (modulator.sample(total_index) * (255.0 / 2.0) + 255.0 / 2.0).round() as u8
            })
            .collect::<Vec<_>>();

        tx.send((THREADS, grayscale_chunk)).unwrap();
    }
    drop(tx);

    let mut grayscale_chunks: Vec<(u32, Vec<u8>)> = rx.iter().collect();
    grayscale_chunks.sort_by_key(|thread| thread.0);
    let frame_vec = grayscale_chunks
        .iter()
        .fold(Vec::new(), |acc, chunk| [&acc[..], &chunk.1[..]].concat());

    for pixel in frame.chunks_exact_mut(4).zip(frame_vec) {
        pixel.0[0] = pixel.1;
        pixel.0[1] = pixel.1;
        pixel.0[2] = pixel.1;
        pixel.0[3] = 255;
    }
}

// Converts the index of a visible pixel between 0 and (H_DISPLAY * V_DISPLAY) into an
// index between 0 and (H_TOTAL * V_TOTAL).
fn visible_to_total_index(pixel_index: usize) -> u32 {
    pixel_index as u32
        // Every time pixel_index exceeds H_DISPLAY add the length of an HBlank interval.
        + (pixel_index as u32 / H_DISPLAY) * (H_TOTAL - H_DISPLAY)
        // Same as above but adds the length of a VBlank interval.
        + (pixel_index as u32 / (H_DISPLAY * V_DISPLAY)) * H_TOTAL * (V_TOTAL - V_DISPLAY)
}
