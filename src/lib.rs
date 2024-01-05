const PROGRAM_BEGIN_ADDR : u16 = 0x200;
const NUM_REGISTERS : usize = 16;
const STACK_SIZE : usize = 32;

struct Emulator<'a, D>
where
    D: Fn(i32, i32, i32),
{
    set_pixel: D,
    memory: &'a mut [u8],
    program_counter: u16,
    registers: [u8; NUM_REGISTERS],
    stack: [u16; STACK_SIZE],
    index: u16,
    delay_timer: u8,
    sound_timer: u8
}

impl<'a, D> Emulator<'a, D>
where
    D: Fn(i32, i32, i32)
{
    pub fn new<'b>(set_pixel: D, memory: &'b mut [u8]) -> Emulator<'b, D> {
        Emulator {
            set_pixel,
            memory,
            program_counter: PROGRAM_BEGIN_ADDR,
            registers: [0; NUM_REGISTERS],
            stack: [0; STACK_SIZE],
            index: 0,
            delay_timer: 0,
            sound_timer: 0
        }
    }

    pub fn load_program(&mut self, program: &[u8]) -> Result<(), &str> {
        let max_program_length = (self.memory.len() as u16) - PROGRAM_BEGIN_ADDR;
        if (program.len() as u16) > max_program_length {
            return Err("error");
        }
        self.memory
            .iter_mut() // grab memory mutably
            .skip(PROGRAM_BEGIN_ADDR as usize) // skip to address where program will be written to
            .take(max_program_length as usize) // truncate to program size
            .zip(program)
            .for_each(|(memory_byte, program_byte)| *memory_byte = *program_byte);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
