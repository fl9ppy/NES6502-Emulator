/// 6502 Status Flag Bits
pub const FLAG_NEGATIVE: u8 = 0b1000_0000;
pub const FLAG_OVERFLOW: u8 = 0b0100_0000;
pub const FLAG_UNUSED:   u8 = 0b0010_0000; // always set to 1 when pushed
pub const FLAG_BREAK:    u8 = 0b0001_0000;
pub const FLAG_DECIMAL:  u8 = 0b0000_1000;
pub const FLAG_INTERRUPT:u8 = 0b0000_0100;
pub const FLAG_ZERO:     u8 = 0b0000_0010;
pub const FLAG_CARRY:    u8 = 0b0000_0001;

/// Trait implemented by CPU to manipulate flags.
/// We separate it so cpu.rs stays clean.
pub trait Flags {
    fn get_status(&self) -> u8;
    fn set_status(&mut self, value: u8);

    /// Read a flag bit
    fn get_flag(&self, flag: u8) -> bool {
        (self.get_status() & flag) != 0
    }

    /// Modify a flag bit
    fn set_flag(&mut self, flag: u8, value: bool) {
        let mut s = self.get_status();
        if value {
            s |= flag;
        } else {
            s &= !flag;
        }
        self.set_status(s);
    }

    /// Update Zero + Negative flags based on a byte
    fn update_zn(&mut self, result: u8) {
        // Zero
        self.set_flag(FLAG_ZERO, result == 0);

        // Negative
        self.set_flag(FLAG_NEGATIVE, (result & 0x80) != 0);
    }
}
