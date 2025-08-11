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

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
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

                    self.update_zero_and_negative_flags(self.register_a);  
                }
                0xAA => {
                    // TAX: Transfer the value from the accumulator (register_a) to the X register (register_x)
                    self.register_x = self.register_a;

                    self.update_zero_and_negative_flags(self.register_x);
                }
                0xE8 => {
                    // INX: Increment X by 1
                    self.register_x = self.register_x.wrapping_add(1);

                    self.update_zero_and_negative_flags(self.register_x);
                }
                0x00 => {
                    // Break instruction
                    return;
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
    fn test_5_ops_working_together() {
       let mut cpu = CPU::new();
       cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
 
       assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}

fn main() {
    let mut cpu = CPU::new();
    cpu.interpret(vec![0xA9, 0x10, 0x00]);
    println!("Register A: {}", cpu.register_a);
}
