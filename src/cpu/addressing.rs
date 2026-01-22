use crate::bus::Bus;
use super::CPU;

/// 6502 addressing modes
#[derive(Copy, Clone, Debug)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    IndirectJmp,
    Relative,
}

/// Value or address produced by addressing-modes
#[derive(Copy, Clone, Debug)]
pub enum Operand {
    Address(u16),
    Immediate(u8),
    Accumulator,
    Relative(i8),
}

impl CPU {
    /// Reads a little-endian u16
    #[inline]
    fn read_u16<B: Bus>(&self, bus: &B, addr: u16) -> u16 {
        let lo = bus.read(addr) as u16;
        let hi = bus.read(addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }

    /// Computes the operand for an instruction based on addressing mode.
    pub fn resolve_addressing<B: Bus>(
        &mut self,
        bus: &mut B,
        mode: AddressingMode
    ) -> Operand {

        match mode {
            AddressingMode::Implied => {
                Operand::Accumulator
            }

            AddressingMode::Accumulator => {
                self.program_counter = self.program_counter.wrapping_add(1);
                Operand::Accumulator
            }

            AddressingMode::Immediate => {
                let value = bus.read(self.program_counter.wrapping_add(1));
                self.program_counter = self.program_counter.wrapping_add(2);
                Operand::Immediate(value)
            }

            AddressingMode::ZeroPage => {
                let addr = bus.read(self.program_counter.wrapping_add(1)) as u16;
                self.program_counter = self.program_counter.wrapping_add(2);
                Operand::Address(addr)
            }

            AddressingMode::ZeroPageX => {
                let base = bus.read(self.program_counter.wrapping_add(1));
                let addr = base.wrapping_add(self.register_x) as u16;
                self.program_counter = self.program_counter.wrapping_add(2);
                Operand::Address(addr & 0x00FF)
            }

            AddressingMode::ZeroPageY => {
                let base = bus.read(self.program_counter.wrapping_add(1));
                let addr = base.wrapping_add(self.register_y) as u16;
                self.program_counter = self.program_counter.wrapping_add(2);
                Operand::Address(addr & 0x00FF)
            }

            AddressingMode::Absolute => {
                let lo = bus.read(self.program_counter.wrapping_add(1)) as u16;
                let hi = bus.read(self.program_counter.wrapping_add(2)) as u16;
                let addr = (hi << 8) | lo;
                self.program_counter = self.program_counter.wrapping_add(3);
                Operand::Address(addr)
            }

            AddressingMode::AbsoluteX => {
                let lo = bus.read(self.program_counter.wrapping_add(1)) as u16;
                let hi = bus.read(self.program_counter.wrapping_add(2)) as u16;
                let base = (hi << 8) | lo;
                let addr = base.wrapping_add(self.register_x as u16);
                self.program_counter = self.program_counter.wrapping_add(3);
                Operand::Address(addr)
            }

            AddressingMode::AbsoluteY => {
                let lo = bus.read(self.program_counter.wrapping_add(1)) as u16;
                let hi = bus.read(self.program_counter.wrapping_add(2)) as u16;
                let base = (hi << 8) | lo;
                let addr = base.wrapping_add(self.register_y as u16);
                self.program_counter = self.program_counter.wrapping_add(3);
                Operand::Address(addr)
            }

            AddressingMode::IndirectX => {
                let zp = bus.read(self.program_counter.wrapping_add(1));
                let ptr = zp.wrapping_add(self.register_x) as u16 & 0x00FF;
                let lo = bus.read(ptr) as u16;
                let hi = bus.read((ptr.wrapping_add(1) & 0x00FF)) as u16;
                let addr = (hi << 8) | lo;
                self.program_counter = self.program_counter.wrapping_add(2);
                Operand::Address(addr)
            }

            AddressingMode::IndirectY => {
                let zp = bus.read(self.program_counter.wrapping_add(1)) as u16;
                let lo = bus.read(zp & 0x00FF) as u16;
                let hi = bus.read((zp.wrapping_add(1) & 0x00FF)) as u16;
                let base = (hi << 8) | lo;
                let addr = base.wrapping_add(self.register_y as u16);
                self.program_counter = self.program_counter.wrapping_add(2);
                Operand::Address(addr)
            }

            AddressingMode::IndirectJmp => {
                let lo_ptr = bus.read(self.program_counter.wrapping_add(1)) as u16;
                let hi_ptr = bus.read(self.program_counter.wrapping_add(2)) as u16;
                let ptr = (hi_ptr << 8) | lo_ptr;

                // 6502 bug: high byte wraps around page
                let lo = bus.read(ptr) as u16;
                let hi_addr = if (ptr & 0x00FF) == 0x00FF {
                    ptr & 0xFF00
                } else {
                    ptr.wrapping_add(1)
                };
                let hi = bus.read(hi_addr) as u16;

                self.program_counter = self.program_counter.wrapping_add(3);
                Operand::Address((hi << 8) | lo)
            }

            AddressingMode::Relative => {
                let offset = bus.read(self.program_counter.wrapping_add(1)) as i8;
                self.program_counter = self.program_counter.wrapping_add(2);
                Operand::Relative(offset)
            }
        }
    }
}
