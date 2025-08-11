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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::{Bus, Ram}; // Import the Bus trait and Ram struct

    // Helper function to set up CPU and RAM with a program loaded at 0x0000
    fn setup_cpu_with_program(program: &[u8]) -> (CPU, Ram) {
        let mut ram = Ram::new();
        ram.load(0x0000, program);
        let cpu = CPU::new();
        (cpu, ram)
    }

    #[test]
    fn test_lda_loads_value_and_sets_flags() {
        let (mut cpu, mut ram) = setup_cpu_with_program(&[0xA9, 0x00, 0x00]); // LDA #$00; BRK
        cpu.run(&mut ram);

        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.status & 0b0000_0010, 0b0000_0010); // Zero flag set

        let (mut cpu, mut ram) = setup_cpu_with_program(&[0xA9, 0x80, 0x00]); // LDA #$80; BRK
        cpu.run(&mut ram);

        assert_eq!(cpu.register_a, 0x80);
        assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000); // Negative flag set
    }

    #[test]
    fn test_tax_transfers_register_and_flags() {
        // Program: LDA #$7F; TAX; BRK
        let program = [0xA9, 0x7F, 0xAA, 0x00];
        let (mut cpu, mut ram) = setup_cpu_with_program(&program);
        cpu.run(&mut ram);

        assert_eq!(cpu.register_x, 0x7F);
        assert_eq!(cpu.status & 0b0000_0010, 0); // Zero flag cleared
        assert_eq!(cpu.status & 0b1000_0000, 0); // Negative flag cleared

        // Program: LDA #$00; TAX; BRK
        let program = [0xA9, 0x00, 0xAA, 0x00];
        let (mut cpu, mut ram) = setup_cpu_with_program(&program);
        cpu.run(&mut ram);

        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.status & 0b0000_0010, 0b0000_0010); // Zero flag set
    }

    #[test]
    fn test_inx_increments_and_wraps() {
        // Program: INX; BRK
        let program = [0xE8, 0x00];

        let mut ram = Ram::new();
        ram.load(0x0000, &program);

        let mut cpu = CPU::new();
        cpu.register_x = 0xFF; // Starting at max value

        cpu.run(&mut ram);

        assert_eq!(cpu.register_x, 0x00); // Wrapped to 0
        assert_eq!(cpu.status & 0b0000_0010, 0b0000_0010); // Zero flag set

        // Test negative flag after increment
        let mut ram = Ram::new();
        ram.load(0x0000, &program);

        let mut cpu = CPU::new();
        cpu.register_x = 0x7F;

        cpu.run(&mut ram);

        assert_eq!(cpu.register_x, 0x80);
        assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000); // Negative flag set
    }
    
    #[test]
    fn test_sta_absolute_writes_register_a_to_memory() {
        use crate::bus::Ram;

        let mut ram = Ram::new();
        let mut cpu = CPU::new();

        cpu.register_a = 0x42;

        // Program: STA $1234; BRK
        // 0x8D 0x34 0x12 0x00
        ram.load(0x0000, &[0x8D, 0x34, 0x12, 0x00]);

        cpu.run(&mut ram);

        assert_eq!(ram.read(0x1234), 0x42);
    }

    #[test]
    fn test_lda_absolute_loads_value_and_flags() {
        let mut ram = Ram::new();
        let mut cpu = CPU::new();

        // Write 0x99 to memory address 0x2000
        ram.write(0x2000, 0x99);

        // Program: LDA $2000; BRK
        ram.load(0x0000, &[0xAD, 0x00, 0x20, 0x00]);

        cpu.run(&mut ram);

        assert_eq!(cpu.register_a, 0x99);
        assert_eq!(cpu.status & 0b1000_0000, 0b1000_0000); // negative flag set
        assert_eq!(cpu.status & 0b0000_0010, 0); // zero flag clear
    }

    #[test]
    fn test_jmp_absolute_jumps_to_address() {
        let mut ram = Ram::new();
        let mut cpu = CPU::new();

        ram.load(0x0000, &[0x4C, 0x05, 0x00, 0xA9, 0x01, 0xA9, 0x42, 0x00]);

        cpu.run(&mut ram);

        assert_eq!(cpu.register_a, 0x42);
    }
}

#[cfg(test)]
mod branch_tests {
    use super::*;
    use crate::bus::{Bus, Ram};

    fn run_program_and_get_pc(program: &[u8]) -> u16 {
        let mut ram = Ram::new();
        let mut cpu = CPU::new();
        ram.load(0x0000, program);
        cpu.run(&mut ram);
        cpu.program_counter
    }

    fn set_status(cpu: &mut CPU, flag_mask: u8, set: bool) {
        if set {
            cpu.status |= flag_mask;
        } else {
            cpu.status &= !flag_mask;
        }
    }

    #[test]
    fn test_beq_taken_and_not_taken() {
        let mut ram = Ram::new();
        let mut cpu = CPU::new();

        // Set zero flag manually for test
        set_status(&mut cpu, 0b0000_0010, true);

        // Program: BEQ +2; BRK
        ram.load(0x0000, &[0xF0, 0x02, 0x00]);
        cpu.run(&mut ram);
        // Should jump 2 bytes ahead (skip BRK)
        assert_eq!(cpu.program_counter, 0x0004);

        // Clear zero flag and run same program
        cpu = CPU::new();
        set_status(&mut cpu, 0b0000_0010, false);
        ram.load(0x0000, &[0xF0, 0x02, 0x00]);
        cpu.run(&mut ram);
        // Should not jump, program counter after BRK (1 + 1 + 1 bytes)
        assert_eq!(cpu.program_counter, 0x0002);
    }

    #[test]
    fn test_bne_taken_and_not_taken() {
        let mut ram = Ram::new();
        let mut cpu = CPU::new();

        // Zero flag clear means branch taken
        set_status(&mut cpu, 0b0000_0010, false);

        ram.load(0x0000, &[0xD0, 0x02, 0x00]);
        cpu.run(&mut ram);
        assert_eq!(cpu.program_counter, 0x0004);

        // Zero flag set means branch NOT taken
        cpu = CPU::new();
        set_status(&mut cpu, 0b0000_0010, true);

        ram.load(0x0000, &[0xD0, 0x02, 0x00]);
        cpu.run(&mut ram);
        assert_eq!(cpu.program_counter, 0x0002);
    }

    #[test]
    fn test_bcc_bcs() {
        let mut ram = Ram::new();
        let mut cpu = CPU::new();

        // Clear carry flag => branch taken
        set_status(&mut cpu, 0b0000_0001, false);
        ram.load(0x0000, &[0x90, 0x02, 0x00]);
        cpu.run(&mut ram);
        assert_eq!(cpu.program_counter, 0x0004);

        // Set carry flag => branch not taken
        cpu = CPU::new();
        set_status(&mut cpu, 0b0000_0001, true);
        ram.load(0x0000, &[0x90, 0x02, 0x00]);
        cpu.run(&mut ram);
        assert_eq!(cpu.program_counter, 0x0002);

        // Now test BCS (branch if carry set)
        cpu = CPU::new();
        set_status(&mut cpu, 0b0000_0001, true);
        ram.load(0x0000, &[0xB0, 0x02, 0x00]);
        cpu.run(&mut ram);
        assert_eq!(cpu.program_counter, 0x0004);

        // Carry clear means branch NOT taken
        cpu = CPU::new();
        set_status(&mut cpu, 0b0000_0001, false);
        ram.load(0x0000, &[0xB0, 0x02, 0x00]);
        cpu.run(&mut ram);
        assert_eq!(cpu.program_counter, 0x0002);
    }

    #[test]
    fn test_bmi_bpl() {
        let mut ram = Ram::new();
        let mut cpu = CPU::new();

        // Negative flag set => BMI branch taken
        set_status(&mut cpu, 0b1000_0000, true);
        ram.load(0x0000, &[0x30, 0x02, 0x00]);
        cpu.run(&mut ram);
        assert_eq!(cpu.program_counter, 0x0004);

        // Negative flag clear => BMI not taken
        cpu = CPU::new();
        set_status(&mut cpu, 0b1000_0000, false);
        ram.load(0x0000, &[0x30, 0x02, 0x00]);
        cpu.run(&mut ram);
        assert_eq!(cpu.program_counter, 0x0002);

        // Negative flag clear => BPL branch taken
        cpu = CPU::new();
        set_status(&mut cpu, 0b1000_0000, false);
        ram.load(0x0000, &[0x10, 0x02, 0x00]);
        cpu.run(&mut ram);
        assert_eq!(cpu.program_counter, 0x0004);

        // Negative flag set => BPL branch NOT taken
        cpu = CPU::new();
        set_status(&mut cpu, 0b1000_0000, true);
        ram.load(0x0000, &[0x10, 0x02, 0x00]);
        cpu.run(&mut ram);
        assert_eq!(cpu.program_counter, 0x0002);
    }
}


