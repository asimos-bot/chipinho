use std::env;
use std::error::Error;
use std::time::Instant;

use chipinho::constants::{DISPLAY_WIDTH, DISPLAY_HEIGHT, NUM_KEYS};

use chipinho::emulator::Emulator;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

const PIXEL_SIZE : u32 = 10;

fn dummy_texture<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<Texture<'a>, String> {
    enum TextureColor {
        White,
    }
    let mut white_pixel = texture_creator
        .create_texture_target(None, PIXEL_SIZE, PIXEL_SIZE)
        .map_err(|e| e.to_string())?;
    // let's change the textures we just created
    {
        let textures = vec![
            (&mut white_pixel, TextureColor::White),
        ];
        canvas
            .with_multiple_texture_canvas(textures.iter(), |texture_canvas, user_context| {
                texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                texture_canvas.clear();
                match *user_context {
                    TextureColor::White => {
                        for i in 0..PIXEL_SIZE {
                            for j in 0..PIXEL_SIZE {
                                if (i + j) % 4 == 0 {
                                    texture_canvas.set_draw_color(Color::RGB(255, 255, 255));
                                    texture_canvas
                                        .draw_point(Point::new(i as i32, j as i32))
                                        .expect("could not draw point");
                                }
                                if (i + j * 2) % 9 == 0 {
                                    texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                                    texture_canvas
                                        .draw_point(Point::new(i as i32, j as i32))
                                        .expect("could not draw point");
                                }
                            }
                        }
                    }
                };
                // for i in 0..PIXEL_SIZE {
                //     for j in 0..PIXEL_SIZE {
                //         // drawing pixel by pixel isn't very effective, but we only do it once and store
                //         // the texture afterwards so it's still alright!
                //         if (i + j) % 7 == 0 {
                //             // this doesn't mean anything, there was some trial and serror to find
                //             // something that wasn't too ugly
                //             texture_canvas.set_draw_color(Color::RGB(255, 255, 255));
                //             texture_canvas
                //                 .draw_point(Point::new(i as i32, j as i32))
                //                 .expect("could not draw point");
                //         }
                //         if (i + j * 2) % 5 == 0 {
                //             texture_canvas.set_draw_color(Color::RGB(127, 127, 127));
                //             texture_canvas
                //                 .draw_point(Point::new(i as i32, j as i32))
                //                 .expect("could not draw point");
                //         }
                //     }
                // }
            })
            .map_err(|e| e.to_string())?;
    }
    Ok(white_pixel)
}

pub fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let mut filename : String;
    match args.iter().skip(1).next() {
        Some(_filename) => filename = String::from(_filename),
        None => return Err(String::from("need a filename"))
    }
    let program = std::fs::read(&filename).map_err(|e| e.to_string())?;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // the window is the representation of a window in your operating system,
    // however you can only manipulate properties of that window, like its size, whether it's
    // fullscreen, ... but you cannot change its content without using a Canvas or using the
    // `surface()` method.
    let window = video_subsystem
        .window(
            "rust-sdl2 demo: Game of Life",
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
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    // clears the canvas with the color we set in `set_draw_color`.
    canvas.clear();
    // However the canvas has not been updated to the window yet, everything has been processed to
    // an internal buffer, but if we want our buffer to be displayed on the window, we need to call
    // `present`. We need to call this everytime we want to render a new frame on the window.
    canvas.present();

    // this struct manages textures. For lifetime reasons, the canvas cannot directly create
    // textures, you have to create a `TextureCreator` instead.
    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    // Create a "target" texture so that we can use our Renderer with it later
    let white_pixel = dummy_texture(&mut canvas, &texture_creator)?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut emulator = Emulator::new();
    emulator.load_program(&program).map_err(|e| format!("error loading program"))?;
    let mut keypad : [bool; NUM_KEYS] = [false; NUM_KEYS];
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
                } => {
                },
                Event::MouseButtonDown {
                    x,
                    y,
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                },
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
            emulator.tick(&keypad).map_err(|e| format!("error on tick: {:?}", e))?;
            start = Instant::now();
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        // iterate over pixels and get which color to print each square
        emulator
            .vram
            .iter()
            .enumerate()
            .try_for_each(|(index, pixel)| -> Result<(), String> {
                let i = index as u32;
                if *pixel {
                    let x = i % DISPLAY_WIDTH as u32;
                    let y = i / DISPLAY_WIDTH as u32;
                    canvas.copy(
                        &white_pixel,
                        None,
                        Rect::new(
                            (x * PIXEL_SIZE) as i32,
                            (y * PIXEL_SIZE) as i32,
                            PIXEL_SIZE,
                            PIXEL_SIZE,
                        )
                    )?;
                }
                Ok(())
            })?; 
        canvas.present();
    }

    Ok(())
}
