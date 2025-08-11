use crate::bus::Bus;

/// The CPU struct represents the central processing unit.
/// It holds registers and status flags required for execution.
pub struct CPU {
    /// Accumulator register (A), used for arithmetic and logic operations.
    pub register_a: u8,

    /// Index register X, used for indexing and loop counters.
    pub register_x: u8,

    /// Status register holding CPU flags:
    /// - Bit 7: Negative flag (N)
    /// - Bit 1: Zero flag (Z)
    /// - Bit 0: Carry flag (C)
    /// and others (not fully implemented here).
    pub status: u8,
 
    /// Program counter (PC), points to the next instruction address.
    pub program_counter: u16,
}

impl CPU { 
    /// Creates a new CPU instance with all registers and flags initialized to zero.
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }
 
    /// Updates the zero and negative flags based on the `result` byte.
    /// - Zero flag is set if `result` is zero.
    /// - Negative flag is set if the most significant bit (bit 7) is set.
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | 0b0000_0010; // Set zero flag
        } else {
            self.status = self.status & 0b1111_1101; // Clear zero flag
        }

        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000; // Set negative flag
        } else {
            self.status = self.status & 0b0111_1111; // Clear negative flag
        }
    }
    
    /// Adjusts the program counter by a signed offset for branching instructions.
    fn branch(&mut self, offset: i8) {
        let pc = self.program_counter as i32;
        let offset = offset as i32;  
        self.program_counter = (pc + offset) as u16;
    }
  
    /// Runs the CPU emulation loop, fetching and executing instructions from the bus.
    /// The loop continues until a BRK (0x00) instruction is encountered.
    ///
    /// The CPU reads instructions from memory via the Bus trait interface.
    pub fn run(&mut self, bus: &mut impl Bus) {
        loop {
            let opcode = bus.read(self.program_counter);

            match opcode {
                0xA9 => {
                    // LDA Immediate: Load accumulator with immediate value
                    let value = bus.read(self.program_counter.wrapping_add(1));
                    self.program_counter = self.program_counter.wrapping_add(2);
                    self.register_a = value;
                    self.update_zero_and_negative_flags(self.register_a);
                }
                0xAD => {
                    // LDA Absolute: Load accumulator from memory address
                    let lo = bus.read(self.program_counter.wrapping_add(1)) as u16;
                    let hi = bus.read(self.program_counter.wrapping_add(2)) as u16;
                    let addr = (hi << 8) | lo;
                    let value = bus.read(addr);
                    self.program_counter = self.program_counter.wrapping_add(3);
                    self.register_a = value;
                    self.update_zero_and_negative_flags(self.register_a);
                }
                0xAA => {
                    // TAX: Transfer accumulator to X register
                    self.program_counter = self.program_counter.wrapping_add(1);
                    self.register_x = self.register_a;
                    self.update_zero_and_negative_flags(self.register_x);
                }
                0xE8 => {
                    // INX: Increment X register
                    self.program_counter = self.program_counter.wrapping_add(1);
                    self.register_x = self.register_x.wrapping_add(1);
                    self.update_zero_and_negative_flags(self.register_x);
                }
                0x8D => {
                    // STA Absolute: Store accumulator to memory address
                    let lo = bus.read(self.program_counter.wrapping_add(1)) as u16;
                    let hi = bus.read(self.program_counter.wrapping_add(2)) as u16;
                    let addr = (hi << 8) | lo;
                    bus.write(addr, self.register_a);
                    self.program_counter = self.program_counter.wrapping_add(3);
                }
                0x4C => {
                    // JMP Absolute: Jump to new address
                    let lo = bus.read(self.program_counter.wrapping_add(1)) as u16;
                    let hi = bus.read(self.program_counter.wrapping_add(2)) as u16;
                    self.program_counter = (hi << 8) | lo;
                }
                0xF0 => {
                    // BEQ: Branch if equal (zero flag set)
                    let offset = bus.read(self.program_counter.wrapping_add(1)) as i8;
                    self.program_counter = self.program_counter.wrapping_add(2);
                    if self.status & 0b0000_0010 != 0 {
                        self.branch(offset);
                    }
                }
                0xD0 => {
                    // BNE: Branch if not equal (zero flag clear)
                    let offset = bus.read(self.program_counter.wrapping_add(1)) as i8;
                    self.program_counter = self.program_counter.wrapping_add(2);
                    if self.status & 0b0000_0010 == 0 {
                        self.branch(offset);
                    }
                }
                0x90 => {
                    // BCC: Branch if carry clear
                    let offset = bus.read(self.program_counter.wrapping_add(1)) as i8;
                    self.program_counter = self.program_counter.wrapping_add(2);
                    if self.status & 0b0000_0001 == 0 {
                        self.branch(offset);
                    }
                }
                0xB0 => {
                    // BCS: Branch if carry set
                    let offset = bus.read(self.program_counter.wrapping_add(1)) as i8;
                    self.program_counter = self.program_counter.wrapping_add(2);
                    if self.status & 0b0000_0001 != 0 {
                        self.branch(offset);
                    }
                }
                0x30 => {
                    // BMI: Branch if negative set
                    let offset = bus.read(self.program_counter.wrapping_add(1)) as i8;
                    self.program_counter = self.program_counter.wrapping_add(2);
                    if self.status & 0b1000_0000 != 0 {
                        self.branch(offset);
                    }
                }
                0x10 => {
                    // BPL: Branch if negative clear
                    let offset = bus.read(self.program_counter.wrapping_add(1)) as i8;
                    self.program_counter = self.program_counter.wrapping_add(2);
                    if self.status & 0b1000_0000 == 0 {
                        self.branch(offset);
                    }
                }
                0x00 => {
                    // BRK: Break / stop execution
                    return;
                }
                _ => panic!("Opcode {:#x} not implemented", opcode),
            }
        }
    }
}
