use std::env;

use chipinho::{constants::NUM_KEYS, emulator::Emulator};

pub fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let filename: String;
    match args.iter().skip(1).next() {
        Some(_filename) => filename = String::from(_filename),
        None => return Err(String::from("need a filename")),
    }
    let program = std::fs::read(&filename).map_err(|e| e.to_string())?;
    let mut emulator = Emulator::new();
    emulator
        .load_program(&program)
        .map_err(|_e| format!("error loading program"))?;
    let keypad: [bool; NUM_KEYS] = [false; NUM_KEYS];
    loop {
        emulator
            .tick(&keypad)
            .map_err(|e| format!("error on tick: {:?}", e))?;
    }
}
