use NES6502_emulator::{CPU, Ram, Bus}; // Use your crate name and correct module paths

#[test]
fn cpu_loads_and_runs_program_from_ram() {
    let mut cpu = CPU::new();
    let mut ram = Ram::new();

    // Program: LDA #$10; TAX; INX; BRK
    let program = vec![0xA9, 0x10, 0xAA, 0xE8, 0x00];

    // Load program at address 0x0000
    ram.load(0x0000, &program);

    // Run CPU with the RAM as its Bus
    cpu.run(&mut ram);

    // Check registers after running the program
    assert_eq!(cpu.register_a, 0x10, "Register A should be 0x10 after LDA");
    assert_eq!(cpu.register_x, 0x11, "Register X should be 0x11 after TAX + INX");
    assert_eq!(cpu.program_counter, 5, "Program Counter should be at end of program");
}

