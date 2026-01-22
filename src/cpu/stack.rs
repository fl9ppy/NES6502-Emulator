use crate::bus::Bus;
use crate::cpu::cpu::CPU;

/// The 6502 stack lives in page $0100.
/// SP is an 8-bit offset within that page.
///
/// This module provides clean helpers for:
/// - push_byte
/// - pop_byte
/// - push_word
/// - pop_word
///
/// The CPU core simply calls into these.
pub trait StackOps {
    fn stack_address(&self) -> u16;
    fn push_byte<B: Bus>(&mut self, bus: &mut B, value: u8);
    fn pop_byte<B: Bus>(&mut self, bus: &mut B) -> u8;
    fn push_word<B: Bus>(&mut self, bus: &mut B, value: u16);
    fn pop_word<B: Bus>(&mut self, bus: &mut B) -> u16;
}

impl StackOps for CPU {

    #[inline]
    fn stack_address(&self) -> u16 {
        0x0100 | self.stack_pointer as u16
    }

    #[inline]
    fn push_byte<B: Bus>(&mut self, bus: &mut B, value: u8) {
        let addr = self.stack_address();
        bus.write(addr, value);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    #[inline]
    fn pop_byte<B: Bus>(&mut self, bus: &mut B) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let addr = self.stack_address();
        bus.read(addr)
    }

    #[inline]
    fn push_word<B: Bus>(&mut self, bus: &mut B, value: u16) {
        // High byte first (6502 requirement)
        let hi = (value >> 8) as u8;
        let lo = (value & 0xFF) as u8;

        self.push_byte(bus, hi);
        self.push_byte(bus, lo);
    }

    #[inline]
    fn pop_word<B: Bus>(&mut self, bus: &mut B) -> u16 {
        let lo = self.pop_byte(bus) as u16;
        let hi = self.pop_byte(bus) as u16;
        (hi << 8) | lo
    }
}
