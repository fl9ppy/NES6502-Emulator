# NES6502 Emulator (WIP)

A simple, well-documented MOS 6502 CPU emulator written in **Rust**.  
This project focuses on clarity, correctness, and clean low-level implementation rather than performance.  
Every subsystem is written from scratch with detailed comments to make the CPU behavior easy to study and extend.

---

## 🚀 Project Goals

- Build a minimal but accurate 6502 CPU emulator  
- Document each instruction and subsystem clearly  
- Learn Rust by implementing real low-level hardware behavior  
- Create a clean foundation for a future NES emulator  
- Keep the code extremely readable and beginner-friendly

This project is intentionally built step-by-step, with focus on understanding the hardware.

---

## 🧠 Current Features

### ✔ CPU Registers

- Accumulator (A)  
- Index Register (X)  
- Status Register (flags)  
- Program Counter (PC)  
- Stack Pointer (SP)  

### ✔ Instruction Set (Partially Implemented)

Supported instructions so far:

- **LDA** — immediate, absolute  
- **TAX**  
- **INX**  
- **STA** (absolute)  
- **JMP** (absolute)  
- **Branching** instructions:  
  - BEQ, BNE  
  - BCC, BCS  
  - BMI, BPL  
- **Stack instructions**:  
  - PHA, PLA  
  - PHP, PLP  
- **Subroutines**:  
  - JSR, RTS  
- **Interrupt/Return**:  
  - RTI  
- **BRK** (software interrupt)

More opcodes are added gradually and documented thoroughly.

---

## 🧱 Implemented Hardware Behavior

### ✔ Stack (0x0100–0x01FF)
Full implementation including:
- 8-bit wrapping stack pointer  
- Byte and word push/pop  
- Hardware-accurate order (high → low for pushes, low → high for pops)  
- Used in JSR/RTS/RTI/BRK and PHA/PLA/PHP/PLP  

### ✔ Interrupt Handling
- **NMI** (non-maskable interrupt)  
- **IRQ** (maskable interrupt)  
- **BRK** behaves like IRQ  
- Status pushed with B flag behavior  
- Jump to correct vector (0xFFFA / 0xFFFE)

### ✔ Bus Interface
A clean `Bus` trait controls all reads/writes.  
Makes it easy to plug in RAM, ROM, or full NES-style memory later.

---

## 🔧 Code Structure

```
src/
├── cpu.rs   # 6502 CPU implementation
├── bus.rs   # Bus trait and simple RAM bus
└── main.rs  # Example program loader
```

---

## 📚 Example

You can load and run simple test programs by writing them into memory via the Bus.

```rust
let mut cpu = CPU::new();
let mut bus = SimpleBus::new();

// Simple sample program
bus.write(0x8000, 0xA9); // LDA #$42
bus.write(0x8001, 0x42);
bus.write(0x8002, 0x00); // BRK

cpu.program_counter = 0x8000;
cpu.run(&mut bus);
```

---

## 🗺 Roadmap

### Coming Next:
- RESET vector implementation  
- More addressing modes (zero page, indexed, indirect)  
- More arithmetic/logical instructions (ADC, SBC, AND, ORA, EOR)  
- Cycle counting  
- NES-specific PPU/APU integration (future)

---

## 🤝 Contributions

This is a learning-focused project, but contributions or suggestions are welcome — especially around:
- Testing strategies  
- Debugging tools  
- Documentation improvements  

---

## 📜 License

MIT License — free to use, modify, and learn from.

---

If you're following along or learning CPU design, feel free to open an issue or star the repo ❤️
