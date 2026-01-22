use crate::bus::Bus;

use super::decode::DECODE_TABLE;
use super::instructions::InstructionHandler;
use super::addressing::AddressingMode;
use super::stack::*;
use super::flags::*;

/// Main CPU structure: only registers + high-level execution
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,

    pub status: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,

    pub nmi_pending: bool,
    pub irq_pending: bool,

    pub cycles: u64,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,

            status: DEFAULT_FLAGS,
            stack_pointer: 0xFD,
            program_counter: 0,

            nmi_pending: false,
            irq_pending: false,

            cycles: 0,
        }
    }

    pub fn reset<B: Bus>(&mut self, bus: &mut B) {
        let lo = bus.read(0xFFFC) as u16;
        let hi = bus.read(0xFFFD) as u16;
        self.program_counter = (hi << 8) | lo;

        self.stack_pointer = 0xFD;
        self.status = DEFAULT_FLAGS;
    }

    /// Fetch one byte from PC, increment PC.
    #[inline]
    pub fn fetch_byte<B: Bus>(&mut self, bus: &mut B) -> u8 {
        let v = bus.read(self.program_counter);
        self.program_counter = self.program_counter.wrapping_add(1);
        v
    }

    #[inline]
    pub fn fetch_word<B: Bus>(&mut self, bus: &mut B) -> u16 {
        let lo = self.fetch_byte(bus) as u16;
        let hi = self.fetch_byte(bus) as u16;
        (hi << 8) | lo
    }

    /// Decode + execute one instruction
    pub fn run_once<B: Bus>(&mut self, bus: &mut B) {
        let opcode = self.fetch_byte(bus);

        let entry = DECODE_TABLE[opcode as usize];

        (entry.handler)(self, bus, entry.mode);
    }
}
