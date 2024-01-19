pub const PROGRAM_BEGIN_ADDR : u16 = 0x200;
pub const NUM_REGISTERS : usize = 16;
pub const MAX_STACK_SIZE : usize = 32;
pub const NUM_KEYS : usize = 16;
pub const FONT_BEGIN_ADDR : u16 = 0x00;
pub const MEMORY_SIZE : u16 = 4096;
pub const DISPLAY_HEIGHT : u8 = 64;
pub const DISPLAY_WIDTH : u8 = 32;

pub const RANDOM_MULTIPLIER : u8 = 42;
pub const RANDOM_INCREMENT : u8 = 31;
pub const RANDOM_MODULE : u8 = 13;

#[no_mangle]
pub extern "C" fn get_program_begin_addr() -> u16 {
    PROGRAM_BEGIN_ADDR
}

#[no_mangle]
pub extern "C" fn get_num_registers() -> usize {
    NUM_REGISTERS 
}

#[no_mangle]
pub extern "C" fn get_max_stack_size() -> usize {
    MAX_STACK_SIZE 
}

#[no_mangle]
pub extern "C" fn get_num_keys() -> usize {
    NUM_KEYS 
}

#[no_mangle]
pub extern "C" fn get_font_begin_addr() -> u16 {
    FONT_BEGIN_ADDR 
}

#[no_mangle]
pub extern "C" fn get_memory_size() -> u16 {
    MEMORY_SIZE 
}

#[no_mangle]
pub extern "C" fn get_display_height() -> u8 {
    DISPLAY_HEIGHT 
}

#[no_mangle]
pub extern "C" fn get_display_width() -> u8 {
    DISPLAY_WIDTH 
}

#[no_mangle]
pub extern "C" fn get_random_multiplier() -> u8 {
    RANDOM_MULTIPLIER 
}

#[no_mangle]
pub extern "C" fn get_random_increment() -> u8 {
    RANDOM_INCREMENT 
}

#[no_mangle]
pub extern "C" fn get_random_module() -> u8 {
    RANDOM_MODULE 
}

