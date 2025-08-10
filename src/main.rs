pub struct CPU{
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16
}

impl CPU{
    pub fn new() -> Self{
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>){
        // Start execution at the beginning of the program (address 0)
        self.program_counter = 0;

        loop {
            // Fetch the opcode (operation code) at the current program counter position and move to next byte
            // The program_counter is cast to usize because Vec uses usize for indexing
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opscode{
                0xA9 => {
                    // LDA Immediate: Load a byte value directly into the accumulator (register_a)
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
                    self.register_a = param;

                    // Update the zero flag (bit 1) in the status register:
                    // If the loaded value is zero, set the zero flag to 1
                    // Otherwise, clear the zero flag (set to 0)
                    if self.register_a == 0 {
                        self.status = self.status | 0b0000_0010;
                    } else {
                        self.status = self.status & 0b1111_1101;
                    }

                    // Update the negative flag (bit 7) in the status register:
                    // If the highest bit (bit 7) of register_a is set (value is negative in 2's complement), set the negative flag
                    // Otherwise, clear the negative flag
                    if self.register_a & 0b1000_0000 != 0 {
                        self.status = self.status | 0b1000_0000;
                    } else{
                        self.status = self.status & 0b0111_1111;
                    }

                }
                0x00 => {
                    // Break instruction
                    return;
                }

                0xAA => {
                    // TAX: Transfer the value from the accumulator (register_a) to the X register (register_x)
                    self.register_x = self.register_a;

                    // Update the zero flag (bit 1) in the status register:
                    // Set it if register_x is zero, clear it otherwise
                    if self.register_x == 0 {
                        self.status = self.status | 0b0000_0010;
                    } else {
                        self.status = self.status & 0b1111_1101;
                    }

                    // Update the negative flag (bit 7) in the status register:
                    // Set it if the most significant bit of register_x is set (indicating a negative value in 2's complement)
                    if self.register_x & 0b1000_0000 != 0 {
                        self.status = self.status | 0b1000_0000;
                    } else {
                        self.status = self.status & 0b0111_1111;
                    }
                }

                _ => todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xA9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.status & 0b0000_0010, 0);
        assert_eq!(cpu.status & 0b1000_0000, 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xA9, 0x00, 0x00]);
        assert_eq!(cpu.status & 0b0000_0010, 0b0000_0010);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
       let mut cpu = CPU::new();
       cpu.register_a = 10;
       cpu.interpret(vec![0xaa, 0x00]);
 
       assert_eq!(cpu.register_x, 10)
    }
}

fn main() {
    let mut cpu = CPU::new();
    cpu.interpret(vec![0xA9, 0x10, 0x00]);
    println!("Register A: {}", cpu.register_a);
}
