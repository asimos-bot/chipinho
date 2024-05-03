
#[derive(Debug)]
pub enum Error {
    ParseInvalidInstruction(u16),
    OutOfBoundsMemoryAccess(u16),
    NotEnoughMemoryForProgram,
    None
}

impl From<Error> for u32 {
    fn from(value: Error) -> Self {
         match value {
            Error::ParseInvalidInstruction(instr) => 0x10010000 | instr as u32,
            Error::OutOfBoundsMemoryAccess(addr) => 0x10020000 | addr as u32,
            Error::NotEnoughMemoryForProgram => 0x10030000,
            Error::None => 0x0
        }
    }
}

impl Into<Error> for u32 {
    fn into(self) -> Error {
        let error_type : u16 = ((self & 0xFFFF0000) >> 16) as u16;
        let data = (self & 0xFFFF) as u16;
        match error_type {
            0x1 => Error::ParseInvalidInstruction(data),
            0x2 => Error::OutOfBoundsMemoryAccess(data),
            0x3 => Error::NotEnoughMemoryForProgram,
            _ => Error::None
        }
    }
}

#[cfg(target_family = "wasm")]
use wasm_bindgen::JsValue;
#[cfg(target_family = "wasm")]
impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        JsValue::from_f64(u32::from(self) as f64)
    }
}
