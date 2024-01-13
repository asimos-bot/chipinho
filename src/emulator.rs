use crate::{instruction::Instruction, error::Error};
use crate::font::{FONT_SET, FONT_SIZE};

const PROGRAM_BEGIN_ADDR : u16 = 0x200;
const NUM_REGISTERS : usize = 16;
const MAX_STACK_SIZE : usize = 32;
const NUM_KEYS : usize = 16;
const FONT_BEGIN_ADDR : u16 = 0x00;
const MEMORY_SIZE : u16 = 4096;
const DISPLAY_HEIGHT : u8 = 64;
const DISPLAY_WIDTH : u8 = 32;
const RANDOM_MULTIPLIER : u8 = 42;
const RANDOM_INCREMENT : u8 = 31;
const RANDOM_MODULE : u8 = 13;

struct Emulator {
    memory: [u8; MEMORY_SIZE as usize],
    vram: [[bool; DISPLAY_WIDTH as usize]; DISPLAY_HEIGHT as usize],
    program_counter: u16,
    registers: [u8; NUM_REGISTERS],
    stack: [u16; MAX_STACK_SIZE],
    stack_size: u16,
    index: u16,
    delay_timer: u8,
    sound_timer: u8,
    waiting_for_key: Option<usize>,
    last_random_u8: u8
}

impl Emulator {
    #[no_mangle]
    pub extern "C" fn new() -> Self {
        let mut emulator = Emulator {
            memory: [0; MEMORY_SIZE as usize],
            program_counter: PROGRAM_BEGIN_ADDR,
            registers: [0; NUM_REGISTERS],
            stack: [0; MAX_STACK_SIZE],
            index: 0,
            delay_timer: 0,
            sound_timer: 0,
            stack_size: 0,
            waiting_for_key: None,
            vram: [[false; DISPLAY_WIDTH as usize]; DISPLAY_HEIGHT as usize],
            last_random_u8: 123
        };
        // load fonts to memory
        emulator.memory
            .iter_mut()
            .skip(FONT_BEGIN_ADDR as usize)
            .take(FONT_SET.len())
            .zip(FONT_SET.iter())
            .for_each(|(byte, font_data)| *byte = *font_data);
        emulator
    }

    #[no_mangle]
    pub extern "C" fn get_memory(&self) -> [u8; MEMORY_SIZE as usize] {
        self.memory.clone()
    }

    #[no_mangle]
    pub extern "C" fn get_vram(&self) -> [[bool; DISPLAY_WIDTH as usize]; DISPLAY_HEIGHT as usize] {
        self.vram.clone()
    }

    #[no_mangle]
    pub extern "C" fn should_beep(&self) -> bool {
        self.sound_timer > 0
    }

    #[no_mangle]
    pub extern "C" fn load_program(&mut self, program: &[u8]) -> Result<(), Error> {
        let max_program_length = (self.memory.len() as u16) - PROGRAM_BEGIN_ADDR;
        if (program.len() as u16) > max_program_length {
            return Err(Error::NotEnoughMemoryForProgram);
        }
        self.memory
            .iter_mut() // grab memory mutably
            .skip(PROGRAM_BEGIN_ADDR as usize) // skip to address where program will be written to
            .take(program.len()) // truncate to program size
            .zip(program)
            .for_each(|(memory_byte, program_byte)| *memory_byte = *program_byte);
        Ok(())
    }

    fn get_random_u8(&mut self) -> u8 {
        self.last_random_u8 = (RANDOM_MULTIPLIER * self.last_random_u8 + RANDOM_INCREMENT) % RANDOM_MODULE;
        self.last_random_u8
    }

    fn get_opcode(&self) -> Result<Instruction, Error> {
        let first_byte : u8 = self.memory
            .get(self.program_counter as usize)
            .cloned()
            .ok_or_else(|| Error::OutOfBoundsMemoryAccess(self.program_counter))?;
        let second_byte : u8 = self.memory
            .get((self.program_counter + 1) as usize)
            .cloned()
            .ok_or_else(|| Error::OutOfBoundsMemoryAccess(self.program_counter))?;

        let raw_opcode = ((first_byte as u16) << 8) & (second_byte as u16);
        Instruction::parse(raw_opcode)
    }

    #[no_mangle]
    pub extern "C" fn tick(&mut self, keypad: [bool; NUM_KEYS]) -> Result<(), Error> {
        match self.waiting_for_key {
            Some(key_index) => if keypad[key_index] {
                self.waiting_for_key = None;
            } else {
                return Ok(());
            },
            None => return Ok(()),
        };
        let opcode : Instruction = self.get_opcode()?;
        self.run_opcode(opcode, &keypad);
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
        Ok(())
    }

    fn run_opcode(&mut self, opcode : Instruction, keypad: &[bool; NUM_KEYS]) -> () {
        match opcode {
            Instruction::Op0nnn(addr) => {
                self.program_counter = addr as u16;
            },
            Instruction::Op00E0 => {
                self.vram
                    .iter_mut()
                    .for_each(|row| {
                        row
                            .iter_mut()
                            .for_each(|pixel| *pixel = false)
                    });
                self.program_counter += 1;
            },
            Instruction::Op00EE => {
                self.program_counter = self.stack[self.stack_size as usize];
                self.stack_size -= 1
            },
            Instruction::Op1nnn(addr) => {
                self.program_counter = addr as u16;
            },
            Instruction::Op2nnn(addr) => {
                self.stack[self.stack_size as usize] = self.program_counter;
                self.stack_size += 1;
                self.program_counter = addr as u16;
            },
            Instruction::Op3xkk(register_index, value) => {
                if self.registers[register_index as usize] as u16 == value {
                    self.program_counter += 2;
                }
                self.program_counter += 1;
            },
            Instruction::Op4xkk(register_index, value) => {
                if self.registers[register_index as usize] as u16 != value {
                    self.program_counter += 2;
                }
                self.program_counter += 1;
            },
            Instruction::Op5xy0(register_index1, register_index2) => {
                if self.registers[register_index1 as usize] == self.registers[register_index2 as usize] {
                    self.program_counter += 2;
                }
                self.program_counter += 1;
            },
            Instruction::Op6xkk(register_index, value) => {
                self.registers[register_index as usize] = value as u8;
                self.program_counter += 1;
            },
            Instruction::Op7xkk(register_index, value) => {
                self.registers[register_index as usize] += value as u8;
                self.program_counter += 1;
            },
            Instruction::Op8xy0(register_index1, register_index2) => {
                self.registers[register_index1 as usize] = self.registers[register_index2 as usize];
                self.program_counter += 1;
            },
            Instruction::Op8xy1(register_index1, register_index2) => {
                self.registers[register_index1 as usize] |= self.registers[register_index2 as usize];
                self.program_counter += 1;
            },
            Instruction::Op8xy2(register_index1, register_index2) => {
                self.registers[register_index1 as usize] &= self.registers[register_index2 as usize];
                self.program_counter += 1;
            },
            Instruction::Op8xy3(register_index1, register_index2) => {
                self.registers[register_index1 as usize] ^= self.registers[register_index2 as usize];
                self.program_counter += 1;
            },
            Instruction::Op8xy4(register_index1, register_index2) => {
                let result : u16 = self.registers[register_index1 as usize] as u16 + self.registers[register_index2 as usize] as u16;
                self.registers[NUM_REGISTERS - 1] = if result > 255 { 1 } else { 0 };
                self.registers[register_index1 as usize] = result as u8;
                self.program_counter += 1;
            },
            Instruction::Op8xy5(register_index1, register_index2) => {
                let result : u16 = self.registers[register_index1 as usize] as u16 - self.registers[register_index2 as usize] as u16;
                self.registers[NUM_REGISTERS - 1] = if self.registers[register_index1 as usize] > self.registers[register_index2 as usize] { 1 } else { 0 };
                self.registers[register_index1 as usize] = result as u8;
                self.program_counter += 1;
            },
            Instruction::Op8xy6(register_index1, register_index2) => {
                self.registers[NUM_REGISTERS - 1] = if self.registers[register_index2 as usize] & 0x1 > 0 { 1 } else { 0 };
                self.registers[register_index2 as usize] = self.registers[register_index1 as usize] >> 1;
                self.program_counter += 1;
            },
            Instruction::Op8xy7(register_index1, register_index2) => {
                let result : u16 = self.registers[register_index2 as usize] as u16 - self.registers[register_index1 as usize] as u16;
                self.registers[NUM_REGISTERS - 1] = if self.registers[register_index2 as usize] > self.registers[register_index1 as usize] { 1 } else { 0 };
                self.registers[register_index1 as usize] = result as u8;
                self.program_counter += 1;
            },
            Instruction::Op8xyE(register_index1, register_index2) => {
                self.registers[NUM_REGISTERS - 1] = if self.registers[register_index1 as usize] & 0b1000_0000 > 0 { 1 } else { 0 };
                self.registers[register_index2 as usize] = self.registers[register_index1 as usize] << 1;
                self.program_counter += 1;
            },
            Instruction::Op9xy0(register_index1, register_index2) => {
                if self.registers[register_index1 as usize] != self.registers[register_index2 as usize] {
                    self.program_counter += 2;
                }
                self.program_counter += 1;
            },
            Instruction::OpAnnn(addr) => {
                self.index = addr as u16;
                self.program_counter += 1;
            },
            Instruction::OpBnnn(addr) => {
                self.index = addr as u16 + self.registers[0] as u16;
                self.program_counter += 1;
            },
            Instruction::OpCxkk(register_index, value) => {
                self.registers[register_index as usize] = self.get_random_u8() & value as u8;
                self.program_counter += 1;
            },
            Instruction::OpDxyn(x, y, value) => {
                self.memory
                    .iter()
                    .skip(self.index as usize)
                    .take(value as usize)
                    // turn memory in iter of bools (bits)
                    .flat_map(|byte| 
                        (0..8)
                            .into_iter()
                            .map(move |index| (byte & (0b1000_0000 >> index)) > 0)
                    )
                    .zip(
                        // get section of vram we will draw at
                        self.vram
                            .iter_mut()
                            // cut rows (no wrap around)
                            .skip(y as usize)
                            .take(value as usize)
                            // cut columns (no wrap around)
                            .map(|row| {
                                row
                                    .iter_mut()
                                    .skip(x as usize)
                                    .take(8)
                            })
                            .flatten()
                    )
                    .for_each(|(memory_bit, vram_bit)| {
                        if *vram_bit == memory_bit && !*vram_bit {
                            self.registers[NUM_REGISTERS - 1] = 1;
                        }
                        *vram_bit = *vram_bit != memory_bit;
                    });
                self.program_counter += 1;
            },
            Instruction::OpEx9E(register_index) => {
                if keypad[self.registers[register_index as usize] as usize] {
                    self.program_counter += 2;
                }
                self.program_counter += 1;
            },
            Instruction::OpExA1(register_index) => {
                if !keypad[self.registers[register_index as usize] as usize] {
                    self.program_counter += 2;
                }
                self.program_counter += 1;
            },
            Instruction::OpFx07(register_index) => {
                self.registers[register_index as usize] = self.delay_timer;
                self.program_counter += 1;
            },
            Instruction::OpFx0A(register_index) => {
                self.waiting_for_key = Some(self.registers[register_index as usize] as usize);
                self.program_counter += 1;
            },
            Instruction::OpFx15(register_index) => {
                self.delay_timer = self.registers[register_index as usize];
                self.program_counter += 1;
            },
            Instruction::OpFx18(register_index) => {
                self.sound_timer = self.registers[register_index as usize];
                self.program_counter += 1;
            },
            Instruction::OpFx1E(register_index) => {
                self.index = self.index + self.registers[register_index as usize] as u16;
                self.program_counter += 1;
            },
            Instruction::OpFx29(register_index) => {
                let value : usize = (self.registers[register_index as usize] & 0x0F) as usize;
                self.index = (FONT_SET.len() + value * FONT_SIZE) as u16;
                self.program_counter += 1;
            },
            Instruction::OpFx33(register_index) => {
                let mut value = self.registers[register_index as usize];
                let mut digits : [u8; 3] = [0, 0, value % 10];
                value /= 10;
                digits[1] = value % 10;
                value /= 10;
                digits[0] = value;
                self.memory
                    .iter_mut()
                    .skip(self.index as usize)
                    .take(3)
                    .zip(digits)
                    .for_each(|(byte, digit)| {
                        *byte = digit;
                    });
                self.program_counter += 1;
            },
            Instruction::OpFx55(register_index) => {
                self.memory
                    .iter_mut()
                    .skip(self.index as usize)
                    .take(register_index as usize)
                    .zip(self.registers)
                    .for_each(|(byte, register)| {
                        *byte = register
                    });
                self.program_counter += 1;
            },
            Instruction::OpFx65(register_index) => {
                self.memory
                    .iter()
                    .skip(self.index as usize)
                    .take(register_index as usize)
                    .zip(self.registers.iter_mut())
                    .for_each(|(byte, register)| {
                        *register = *byte;
                    });
                self.program_counter += 1;
            },
        }
    }
}

// mod math {
//     mod math_js {
//         #[link(wasm_import_module = "Math")]
//         extern "C" {
//             #[link_name = "random"]
//             pub fn random() -> f64;
//         }
//     }
//     pub fn random() -> f64 {
//         unsafe { math_js::random() }
//     }
// }
//
//
// #[export_name = "add"]
// #[no_mangle]
// pub extern "C" fn add(left: f64, right: f64) -> f64 {
//     left + right + math::random()
// }
