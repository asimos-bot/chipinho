use std::env;
use std::time::Instant;

use chipinho::constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH, NUM_KEYS};

use chipinho::emulator::Emulator;
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const PIXEL_SIZE: u32 = 10;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let filename: String;
    match args.iter().skip(1).next() {
        Some(_filename) => filename = String::from(_filename),
        None => return Err(String::from("need a filename")),
    }
    let program = std::fs::read(&filename).map_err(|e| e.to_string())?;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let audio_subsystem = sdl_context.audio()?;

    let desired_spec = AudioSpecDesired {
        freq: Some(44_100),
        channels: Some(1),
        samples: None,
    };

    let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| SquareWave {
        phase_inc: 440.0 / spec.freq as f32,
        phase: 0.0,
        volume: 0.25,
    })?;

    // the window is the representation of a window in your operating system,
    // however you can only manipulate properties of that window, like its size, whether it's
    // fullscreen, ... but you cannot change its content without using a Canvas or using the
    // `surface()` method.
    let window = video_subsystem
        .window(
            "chipinho demo: SDL",
            PIXEL_SIZE * DISPLAY_WIDTH as u32,
            PIXEL_SIZE * DISPLAY_HEIGHT as u32,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // the canvas allows us to both manipulate the property of the window and to change its content
    // via hardware or software rendering. See CanvasBuilder for more info.
    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut emulator = Emulator::new();
    emulator
        .load_program(&program)
        .map_err(|e| format!("error loading program"))?;
    let mut keypad: [bool; NUM_KEYS] = [false; NUM_KEYS];
    let mut start = Instant::now();

    'running: loop {
        // get the inputs here
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } => {}
                Event::MouseButtonDown {
                    x,
                    y,
                    mouse_btn: MouseButton::Left,
                    ..
                } => {}
                Event::KeyDown {
                    keycode: Some(Keycode::Num1),
                    repeat: true,
                    ..
                } => keypad[0x01] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Num2),
                    ..
                } => keypad[0x02] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    ..
                } => keypad[0x03] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Num4),
                    ..
                } => keypad[0x0C] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => keypad[0x04] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => keypad[0x05] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => keypad[0x06] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => keypad[0x0D] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => keypad[0x07] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => keypad[0x08] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => keypad[0x09] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => keypad[0x0E] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => keypad[0x0A] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
                } => keypad[0x00] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => keypad[0x0B] = true,
                Event::KeyDown {
                    keycode: Some(Keycode::V),
                    ..
                } => keypad[0x0F] = true,
                Event::KeyUp {
                    keycode: Some(Keycode::Num1),
                    repeat: false,
                    ..
                } => keypad[0x01] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Num2),
                    ..
                } => keypad[0x02] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Num3),
                    ..
                } => keypad[0x03] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Num4),
                    ..
                } => keypad[0x0C] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Q),
                    ..
                } => keypad[0x04] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => keypad[0x05] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::E),
                    ..
                } => keypad[0x06] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::R),
                    ..
                } => keypad[0x0D] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => keypad[0x07] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => keypad[0x08] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => keypad[0x09] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::F),
                    ..
                } => keypad[0x0E] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Z),
                    ..
                } => keypad[0x0A] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::X),
                    ..
                } => keypad[0x00] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::C),
                    ..
                } => keypad[0x0B] = false,
                Event::KeyUp {
                    keycode: Some(Keycode::V),
                    ..
                } => keypad[0x0F] = false,
                _ => {}
            }
        }
        // update the game loop here
        if (Instant::now() - start).as_millis() >= 16 {
            // println!("opcode: {}", emulator.get_opcode().map_err(|e| format!("error getting opcode"))?.to_str());
            emulator
                .tick(&keypad)
                .map_err(|e| format!("error on tick: {:?}", e))?;
            start = Instant::now();
        }


        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::GRAY);
        // iterate over pixels and get which color to print each square
        canvas.fill_rects(
            &emulator
            .vram
            .iter()
            .enumerate()
            .filter_map(|(index, pixel)| -> Option<Rect> {
                let i = index as u32;
                if *pixel {
                    let x = i % DISPLAY_WIDTH as u32;
                    let y = i / DISPLAY_WIDTH as u32;
                    return Some(Rect::new(
                        (x * PIXEL_SIZE) as i32,
                        (y * PIXEL_SIZE) as i32,
                        PIXEL_SIZE,
                        PIXEL_SIZE,
                    ));
                }
                None
            })
            .collect::<Vec<Rect>>()
            .as_slice()
        )?;
        canvas.present();
        if emulator.should_beep() {
            audio_device.resume();
        } else {
            audio_device.pause();
        }
    }

    Ok(())
}
