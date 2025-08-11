/// The Bus trait defines how the CPU interacts with memory or devices.
/// It requires two functions:
/// - `read`: read a byte (u8) from a 16-bit address (u16)
/// - `write`: write a byte (u8) to a 16-bit address (u16)
pub trait Bus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}

/// Ram struct represents the computer's memory.
/// It holds 64KB (65536 bytes) of data.
pub struct Ram {
    mem: [u8; 65536],  // Memory array: each element is one byte
}

impl Ram {
    /// Creates a new Ram instance with all bytes initialized to zero.
    pub fn new() -> Self {
        Ram { mem: [0; 65536] } // The array is filled with zeros
    }

    /// Loads a program (slice of bytes) into memory starting at `start_addr`.
    /// Copies the program bytes into `mem` at the right position.
    pub fn load(&mut self, start_addr: u16, program: &[u8]) {
        // Convert start address to usize so it can index into the array
        let start = start_addr as usize;

        // Calculate the end index (start + length of program)
        let end = start + program.len();

        // Copy program bytes into the RAM memory array slice
        self.mem[start..end].copy_from_slice(program);
    }
}

// Implement the Bus trait for Ram so the CPU can read/write memory through it.
impl Bus for Ram {
    /// Reads a byte from memory at the given address.
    fn read(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    /// Writes a byte to memory at the given address.
    fn write(&mut self, addr: u16, data: u8) {
        self.mem[addr as usize] = data;
    }
}
