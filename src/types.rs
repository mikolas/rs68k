/// Special processor states (must match m68000gen.py order)
pub const S_RESET: u16 = 0;
pub const S_BUS_ERROR: u16 = 1;
pub const S_ADDRESS_ERROR: u16 = 2;
pub const S_DOUBLE_FAULT: u16 = 3;
pub const S_INTERRUPT: u16 = 4;
pub const S_TRACE: u16 = 5;
pub const S_ILLEGAL: u16 = 6;
pub const S_PRIVILEGE: u16 = 7;
pub const S_LINEA: u16 = 8;
pub const S_LINEF: u16 = 9;
/// SR flags
pub const SR_C: u16 = 0x0001;
pub const SR_V: u16 = 0x0002;
pub const SR_Z: u16 = 0x0004;
pub const SR_N: u16 = 0x0008;
pub const SR_X: u16 = 0x0010;
pub const SR_I: u16 = 0x0700;
pub const SR_S: u16 = 0x2000;
pub const SR_T: u16 = 0x8000;

pub const SR_CCR: u16 = SR_C | SR_V | SR_Z | SR_N | SR_X;
pub const SR_SR: u16 = SR_I | SR_S | SR_T;

/// SSW (Special Status Word) contents
pub const SSW_DATA: u16 = 0x01;
pub const SSW_PROGRAM: u16 = 0x02;
pub const SSW_CPU: u16 = 0x03;
pub const SSW_S: u16 = 0x04;
pub const SSW_N: u16 = 0x08;
pub const SSW_R: u16 = 0x10;
pub const SSW_CRITICAL: u16 = 0x20;

/// Bus trait — the emulator implements this to provide memory access.
pub trait Bus {
    /// Read word from program/opcode space.
    fn read_program(&mut self, addr: u32) -> u16;
    /// Read word from data space.
    fn read_data(&mut self, addr: u32) -> u16;
    /// Write word to data space.
    fn write_data(&mut self, addr: u32, data: u16);
    /// Write byte to data space. addr bit 0 selects high (0) or low (1) byte.
    fn write_data_byte(&mut self, addr: u32, data: u8);
    /// Read from CPU space (interrupt acknowledge).
    fn read_cpu(&mut self, addr: u32) -> u16;
}
