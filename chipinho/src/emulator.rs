#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    constants::*,
    error::Error,
    font::{FONT_SET, FONT_SIZE},
    instruction::Instruction,
};

#[derive(Clone, Copy)]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[cfg_attr(not(target_family = "wasm"), repr(C))]
pub struct WaitingKey {
    register_index: usize,
    key_index: usize,
    has_been_pressed: bool,
}

#[derive(Clone, Copy)]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[cfg_attr(not(target_family = "wasm"), repr(C))]
pub struct Emulator {
    pub program_counter: u16,
    pub index: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub waiting_key: Option<WaitingKey>,
    pub last_random_u8: u8,
    pub stack_size: u16,

    registers: [u8; NUM_REGISTERS],
    stack: [u16; MAX_STACK_SIZE],
    memory: [u8; MEMORY_SIZE as usize],
    vram: [u8; DISPLAY_WIDTH as usize * DISPLAY_HEIGHT as usize],

    #[cfg(target_family = "wasm")]
    pub display_height: u8,
    #[cfg(target_family = "wasm")]
    pub display_width: u8,
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
impl Emulator {
    #[cfg_attr(not(target_family = "wasm"), no_mangle)]
    #[cfg_attr(target_family = "wasm", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        let mut emulator = Emulator {
            program_counter: PROGRAM_BEGIN_ADDR,
            index: 0,
            delay_timer: 0,
            sound_timer: 0,
            waiting_key: None,
            last_random_u8: 123,
            stack_size: 0,
            registers: [0; NUM_REGISTERS],
            stack: [0; MAX_STACK_SIZE],
            memory: [0; MEMORY_SIZE as usize],
            vram: [0; DISPLAY_WIDTH as usize * DISPLAY_HEIGHT as usize],

            #[cfg(target_family = "wasm")]
            display_height: DISPLAY_HEIGHT,
            #[cfg(target_family = "wasm")]
            display_width: DISPLAY_WIDTH,
        };

        // load fonts to memory
        emulator
            .memory
            .iter_mut()
            .skip(FONT_BEGIN_ADDR as usize)
            .take(FONT_SET.len())
            .zip(FONT_SET.iter())
            .for_each(|(byte, font_data)| *byte = *font_data);
        emulator
    }

    #[cfg_attr(not(target_family = "wasm"), no_mangle)]
    pub extern "C" fn should_beep(&self) -> bool {
        self.sound_timer > 0
    }

    #[cfg_attr(not(target_family = "wasm"), no_mangle)]
    pub extern "C" fn load_program(&mut self, program: &[u8]) -> u32 {
        let max_program_length = (self.memory.len() as u16) - PROGRAM_BEGIN_ADDR;
        if (program.len() as u16) > max_program_length {
            return Error::NotEnoughMemoryForProgram.into();
        }
        self.program_counter = PROGRAM_BEGIN_ADDR;
        self.memory
            .iter_mut() // grab memory mutably
            .skip(PROGRAM_BEGIN_ADDR as usize) // skip to address where program will be written to
            .take(program.len()) // truncate to program size
            .zip(program)
            .for_each(|(memory_byte, program_byte)| *memory_byte = *program_byte);
        0
    }

    fn get_random_u8(&mut self) -> u8 {
        self.last_random_u8 = (RANDOM_MULTIPLIER
            .wrapping_mul(self.last_random_u8)
            .wrapping_add(RANDOM_INCREMENT))
            % RANDOM_MODULE;
        self.last_random_u8
    }

    #[cfg(target_family = "wasm")]
    pub fn get_vram(&self) -> Vec<u8> {
        self.vram.to_vec()
    }

    #[no_mangle]
    #[cfg(not(target_family = "wasm"))]
    pub extern "C" fn get_vram(&self) -> &[u8] {
        &self.vram
    }

    // #[cfg_attr(not(target_family = "wasm"), no_mangle)]
    fn get_opcode(&self) -> Result<Instruction, Error> {
        let first_byte: u8 = self
            .memory
            .get(self.program_counter as usize)
            .cloned()
            .ok_or_else(|| Error::OutOfBoundsMemoryAccess(self.program_counter))?;
        let second_byte: u8 = self
            .memory
            .get((self.program_counter + 1) as usize)
            .cloned()
            .ok_or_else(|| Error::OutOfBoundsMemoryAccess(self.program_counter))?;

        let raw_opcode = ((first_byte as u16) << 8) | (second_byte as u16);
        Instruction::parse(raw_opcode)
    }

    #[cfg_attr(not(target_family = "wasm"), no_mangle)]
    pub extern "C" fn tick(&mut self, keypad: &[u8]) -> u32 {
        // update timers (even if blocking for keypress)
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        match self.waiting_key {
            Some(WaitingKey {
                register_index,
                key_index: _,
                has_been_pressed: false,
            }) => {
                if let Some(key_index) = keypad.iter().take(NUM_KEYS).position(|pressed| *pressed != 0) {
                    self.waiting_key = Some(WaitingKey {
                        register_index,
                        key_index,
                        has_been_pressed: true
                    });
                    return 0;
                }
            },
            Some(WaitingKey {
                register_index,
                key_index,
                has_been_pressed: true,
            }) => {
                if keypad[key_index] == 0 {
                    self.registers[register_index] = key_index as u8;
                    self.program_counter += 2;
                    self.waiting_key = None;
                }
            }
            None => {}
        };
        match self.get_opcode() {
            Ok(opcode) => {
                self.run_opcode(opcode, &keypad);
                0
            },
            Err(err) => err.into(),
        }
    }

    fn run_opcode(&mut self, opcode: Instruction, keypad: &[u8]) -> () {
        match opcode {
            Instruction::Op0nnn(addr) => {
                self.program_counter = addr as u16;
            }
            Instruction::Op00E0 => {
                self.vram.iter_mut().for_each(|pixel| *pixel = 0);
                self.program_counter += 2;
            }
            Instruction::Op00EE => {
                self.stack_size -= 1;
                self.program_counter = self.stack[self.stack_size as usize];
            }
            Instruction::Op1nnn(addr) => {
                self.program_counter = addr as u16;
            }
            Instruction::Op2nnn(addr) => {
                self.stack[self.stack_size as usize] = self.program_counter + 2;
                self.stack_size += 1;
                self.program_counter = addr as u16;
            }
            Instruction::Op3xkk(register_index, value) => {
                if self.registers[register_index as usize] == value as u8 {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
            }
            Instruction::Op4xkk(register_index, value) => {
                if self.registers[register_index as usize] as u16 != value {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
            }
            Instruction::Op5xy0(register_index1, register_index2) => {
                if self.registers[register_index1 as usize]
                    == self.registers[register_index2 as usize]
                {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
            }
            Instruction::Op6xkk(register_index, value) => {
                self.registers[register_index as usize] = value as u8;
                self.program_counter += 2;
            }
            Instruction::Op7xkk(register_index, value) => {
                self.registers[register_index as usize] =
                    self.registers[register_index as usize].wrapping_add(value as u8);
                self.program_counter += 2;
            }
            Instruction::Op8xy0(register_index1, register_index2) => {
                self.registers[register_index1 as usize] = self.registers[register_index2 as usize];
                self.program_counter += 2;
            }
            Instruction::Op8xy1(register_index1, register_index2) => {
                self.registers[register_index1 as usize] |=
                    self.registers[register_index2 as usize];
                self.registers[NUM_REGISTERS - 1] = 0;
                self.program_counter += 2;
            }
            Instruction::Op8xy2(register_index1, register_index2) => {
                self.registers[register_index1 as usize] &=
                    self.registers[register_index2 as usize];
                self.registers[NUM_REGISTERS - 1] = 0;
                self.program_counter += 2;
            }
            Instruction::Op8xy3(register_index1, register_index2) => {
                self.registers[register_index1 as usize] ^=
                    self.registers[register_index2 as usize];
                self.registers[NUM_REGISTERS - 1] = 0;
                self.program_counter += 2;
            }
            Instruction::Op8xy4(register_index1, register_index2) => {
                let result: u16 = self.registers[register_index1 as usize] as u16
                    + self.registers[register_index2 as usize] as u16;
                self.registers[register_index1 as usize] = result as u8;
                self.registers[NUM_REGISTERS - 1] = if result > 255 { 1 } else { 0 };
                self.program_counter += 2;
            }
            Instruction::Op8xy5(register_index1, register_index2) => {
                let set_vf = if self.registers[register_index1 as usize]
                    >= self.registers[register_index2 as usize]
                {
                    1
                } else {
                    0
                };
                let result: u8 = self.registers[register_index1 as usize]
                    .wrapping_sub(self.registers[register_index2 as usize]);
                self.registers[register_index1 as usize] = result;
                self.registers[NUM_REGISTERS - 1] = set_vf;
                self.program_counter += 2;
            }
            Instruction::Op8xy6(register_index1, register_index2) => {
                let set_vf = if self.registers[register_index2 as usize] & 0x1 > 0 {
                    1
                } else {
                    0
                };
                self.registers[register_index1 as usize] =
                    self.registers[register_index2 as usize] >> 1;
                self.registers[NUM_REGISTERS - 1] = set_vf;
                self.program_counter += 2;
            }
            Instruction::Op8xy7(register_index1, register_index2) => {
                let set_vf = if self.registers[register_index2 as usize]
                    >= self.registers[register_index1 as usize]
                {
                    1
                } else {
                    0
                };
                let result: u8 = self.registers[register_index2 as usize]
                    .wrapping_sub(self.registers[register_index1 as usize]);
                self.registers[register_index1 as usize] = result;
                self.registers[NUM_REGISTERS - 1] = set_vf;
                self.program_counter += 2;
            }
            Instruction::Op8xyE(register_index1, register_index2) => {
                let set_vf = if self.registers[register_index2 as usize] & 0b1000_0000 > 0 {
                    1
                } else {
                    0
                };
                self.registers[register_index1 as usize] =
                    self.registers[register_index2 as usize] << 1;
                self.registers[NUM_REGISTERS - 1] = set_vf;
                self.program_counter += 2;
            }
            Instruction::Op9xy0(register_index1, register_index2) => {
                if self.registers[register_index1 as usize]
                    != self.registers[register_index2 as usize]
                {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
            }
            Instruction::OpAnnn(addr) => {
                self.index = addr as u16;
                self.program_counter += 2;
            }
            Instruction::OpBnnn(addr) => {
                self.program_counter = addr as u16 + self.registers[0] as u16;
            }
            Instruction::OpCxkk(register_index, value) => {
                self.registers[register_index as usize] = self.get_random_u8() & value as u8;
                self.program_counter += 2;
            }
            Instruction::OpDxyn(register_index1, register_index2, value) => {
                let x = self.registers[register_index1 as usize] % DISPLAY_WIDTH;
                let y = self.registers[register_index2 as usize] % DISPLAY_HEIGHT;
                self.registers[NUM_REGISTERS - 1] = 0;
                let value = if y + value < DISPLAY_HEIGHT {
                    value as usize
                } else {
                    DISPLAY_HEIGHT as usize - y as usize
                };
                let max_bit = if x + 8 < DISPLAY_WIDTH {
                    8
                } else {
                    DISPLAY_WIDTH as usize - x as usize
                };
                for byte in 0..value {
                    let py = (y as usize + byte as usize) % DISPLAY_HEIGHT as usize;
                    for bit in 0..max_bit as usize {
                        let px = (x as usize + bit) % DISPLAY_WIDTH as usize;
                        let color =
                            (self.memory[self.index as usize + byte as usize] >> (7 - bit)) & 1;
                        let pixel =
                            &mut self.vram[py as usize * DISPLAY_WIDTH as usize + px as usize];
                        self.registers[NUM_REGISTERS - 1] |= color & (*pixel) as u8;
                        *pixel ^= (color != 0) as u8;
                    }
                }
                self.program_counter += 2;
            }
            Instruction::OpEx9E(register_index) => {
                if keypad[self.registers[register_index as usize] as usize] != 0 {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
            }
            Instruction::OpExA1(register_index) => {
                if keypad[self.registers[register_index as usize] as usize] == 0 {
                    self.program_counter += 2;
                }
                self.program_counter += 2;
            }
            Instruction::OpFx07(register_index) => {
                self.registers[register_index as usize] = self.delay_timer;
                self.program_counter += 2;
            }
            Instruction::OpFx0A(register_index) => {
                self.waiting_key = Some(WaitingKey {
                    register_index: register_index as usize,
                    key_index: 0,
                    has_been_pressed: false
                });
            }
            Instruction::OpFx15(register_index) => {
                self.delay_timer = self.registers[register_index as usize];
                self.program_counter += 2;
            }
            Instruction::OpFx18(register_index) => {
                self.sound_timer = self.registers[register_index as usize];
                self.program_counter += 2;
            }
            Instruction::OpFx1E(register_index) => {
                self.index = self.index + self.registers[register_index as usize] as u16;
                self.program_counter += 2;
            }
            Instruction::OpFx29(register_index) => {
                let value: usize = (self.registers[register_index as usize] & 0x0F) as usize;
                self.index = (FONT_SET.len() + value * FONT_SIZE) as u16;
                self.program_counter += 2;
            }
            Instruction::OpFx33(register_index) => {
                let value = self.registers[register_index as usize];
                let digits: [u8; 3] = [value / 100, (value % 100) / 10, value % 10];
                self.memory
                    .iter_mut()
                    .skip(self.index as usize)
                    .take(3)
                    .zip(digits)
                    .for_each(|(byte, digit)| {
                        *byte = digit;
                    });
                self.program_counter += 2;
            }
            Instruction::OpFx55(register_index) => {
                self.memory
                    .iter_mut()
                    .skip(self.index as usize)
                    .take(register_index as usize + 1)
                    .zip(self.registers.iter().take(register_index as usize + 1))
                    .for_each(|(byte, register)| *byte = *register);
                self.index += register_index as u16 + 1;
                self.program_counter += 2;
            }
            Instruction::OpFx65(register_index) => {
                self.memory
                    .iter()
                    .skip(self.index as usize)
                    .take(register_index as usize + 1)
                    .zip(self.registers.iter_mut().take(register_index as usize + 1))
                    .for_each(|(byte, register)| {
                        *register = *byte;
                    });
                self.index += register_index as u16 + 1;
                self.program_counter += 2;
            }
        }
    }
}
