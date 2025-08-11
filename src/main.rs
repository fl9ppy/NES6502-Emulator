mod cpu;
mod bus;

use cpu::CPU;
use bus::{Bus, Ram};

fn main() {
    let mut ram = Ram::new();

    // Load program at address 0x0000
    let program = vec![0xA9, 0x10, 0xAA, 0xE8, 0x00];
    ram.load(0x0000, &program);

    let mut cpu = CPU::new();
    cpu.program_counter = 0x0000; // start at program start

    cpu.run(&mut ram);

    println!("Register A: {}", cpu.register_a);
    println!("Register X: {}", cpu.register_x);
}

