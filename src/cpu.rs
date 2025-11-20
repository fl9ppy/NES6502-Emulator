use crate::bus::Bus;

const NMI_VECTOR: u16 = 0xFFFA;
const RESET_VECTOR: u16 = 0xFFFC;
const IRQ_VECTOR: u16 = 0xFFFE;

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

    /// Stack Pointer (SP), points to the current top of the stack (0x0100-0x01FF).
    /// Stack grows downward; used by PHA/PLA/JSR/RTS/RTI/PHP/PLP instructions.
    pub stack_pointer: u8,

    /// Pending interrupt requests (NMI cannot be masked, IRQ can be).
    pub nmi_pending: bool,
    pub irq_pending: bool,
}

impl CPU { 
    /// Creates a new CPU instance with all registers and flags initialized to zero.
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
            stack_pointer: 0xFD,
        }
    }
    
    /// Requests a maskable interrupt (IRQ). Ignored if I flag is set.
    pub fn trigger_irq(&mut self) {
        self.irq_pending = true;
    }

    /// Handles an interrupt by pushing PC and status, clearing the break bit,
    /// and loading a new PC from the interrupt vector.
    fn handle_interrupt(&mut self, bus: &mut impl Bus, vector: u16){
        // Push PC onto stack (high, then low)
        let pc = self.program_counter;
        self.push_word(bus, pc);

        // Push status (B flag cleared on actual interrupts) 
        let flags = self.status & 0b1110_1111; // Clear B flag
        self.push_byte(bus, flags);

        // Set interrupt disable flag
        self.status |= 0b0000_0100;

        // Load new PC from vector
        let lo = bus.read(vector) as u16;
        let hi = bus.read(vector + 1) as u16;
        self.program_counter = (hi << 8) | lo;
    }

    /// Requests a non-maskable interrupt (NMI). Always taken.
    pub fn trigger_nmi(&mut self) {
        self.nmi_pending = true
    }

    /// Computes the absolute memory address of the stack location pointed by 'stack_pointer'.
    /// Stack resides in page 0x0100 (0x0100 - 0x01FF).
    fn stack_address(&self) -> u16 {
        0x0100 | self.stack_pointer as u16 
    }
    
    /// Pushes a byte onto the stack.
    /// Decrements `stack_pointer` after writing (stack grows downward).
    pub fn push_byte(&mut self, bus: &mut impl Bus, value: u8){
        let addr = self.stack_address();
        bus.write(addr, value);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1) // wrap-around at 0x00 -> 0xFF
    }

    /// Pops a byte from the stack.
    /// Increments `stack_pointer` before reading.
    pub fn pop_byte(&mut self, bus: &mut impl Bus) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1); // wrap-around at 0xFF -> 0x00
        let addr = self.stack_address();
        bus.read(addr)
    }

    /// Pushes a 16-bit value onto the stack.
    /// High byte is pushed first, then low byte.
    /// Used by JSR and interrupt routines.
    pub fn push_word(&mut self, bus: &mut impl Bus, value: u16) {
        let high = (value >> 8) as u8;
        let low = (value & 0xFF) as u8;
        self.push_byte(bus, high);
        self.push_byte(bus, low);
    }

    /// Pops a 16-bit value from the stack.
    /// Low byte is popped first, then high byte.
    /// Used by RTS and RTI instructions.
    pub fn pop_word(&mut self, bus: &mut impl Bus) -> u16 {
        let low = self.pop_byte(bus) as u16;
        let high = self.pop_byte(bus) as u16;
        (high << 8) | low
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
            // Handle interrupts before executing next instruction
            if self.nmi_pending = false;{
                self.nmi_pending = false;
                self.handle_interrupt(bus, NMI_VECTOR);
            } else if self.irq_pending = false; {
                self.irq_pending = false;
                self.handle_interrupt(bus, IRQ_VECTOR);
            }

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
                    // BRK: Force interrupt
                    self.program_counter = self.program_counter.wrapping_add(1);

                    // Push PC and status (break flag set)
                    self.push_word(bus, self.program_counter);
                    self.push_byte(bus, self.status | 0b0001_0000);

                    // Set interrupt disable
                    self.status |= 0b0000_0100;

                    // Jump to IRQ/BRK vector
                    let lo = bus.read(IRQ_VECTOR) as u16;
                    let hi = bus.read(IRQ_VECTOR + 1) as u16;
                    self.program_counter = (hi << 8) | lo;
                }
                0x48 => {
                // PHA: Push accumulator to stack
                    self.push_byte(bus, self.register_a);
                    self.program_counter = self.program_counter.wrapping_add(1);
                },
                0x68 => {
                    // PLA: Pull accumulator from stack
                    self.register_a = self.pop_byte(bus);
                    self.update_zero_and_negative_flags(self.register_a);
                    self.program_counter = self.program_counter.wrapping_add(1);
                },
                0x08 => {
                    // PHP: Push processor status to stack (set B flag + unused)
                    self.push_byte(bus, self.status | 0b0011_0000);
                    self.program_counter = self.program_counter.wrapping_add(1);
                },
                0x28 => {
                    // PLP: Pull processor status from stack
                    self.status = self.pop_byte(bus);
                    self.program_counter = self.program_counter.wrapping_add(1);
                },
                0x20 => {
                    // JSR Absolute: Jump to subroutine
                    let lo = bus.read(self.program_counter.wrapping_add(1)) as u16;
                    let hi = bus.read(self.program_counter.wrapping_add(2)) as u16;
                    let addr = (hi << 8) | lo;
                    // Push return address (PC + 2) onto stack
                    self.push_word(bus, self.program_counter.wrapping_add(2));
                    self.program_counter = addr;
                },
                0x60 => {
                    // RTS: Return from subroutine
                    self.program_counter = self.pop_word(bus).wrapping_add(1);
                },
                0x40 => {
                    // RTI: Return from interrupt
                    self.status = self.pop_byte(bus);          // Restore status flags
                    self.program_counter = self.pop_word(bus); // Restore PC
                }
                _ => panic!("Opcode {:#x} not implemented", opcode),
            }
        }
    }
}
