/// A virtual CPU that implements a subset of CHIP-8 ops.
pub struct CPU {
    op: u16,  // the current operation
    reg: [u8; 16], // 16 registers can be addressed by a single hex val (0-F)
    pc: usize,  // program counter: points to the current position in memory
    mem: [u8; 4096], // 4K of RAM (0x1000)
}

impl CPU {

    /// instantiates a default CPU
    pub fn new() -> CPU {
        CPU { op: 0, reg: [0; 16], pc: 0, mem: [0; 4096] }
    }

    /// read in the current operation referenced by the program_counter
    fn read_opcode(&self) -> u16 {
        let op_byte1 = self.mem[self.pc] as u16;      // 0b00000000XXXXXXXX
        let op_byte2 = self.mem[self.pc + 1] as u16;  // 0b00000000YYYYYYYY

        op_byte1 << 8 | op_byte2  // 0bXXXXXXXXYYYYYYYY

    }
    /// decode the passed CHIP-8 opcode into its components:
    /// n ---> 0x000F (number of bytes)
    /// x ---> 0x0F00 (CPU register)
    /// y ---> 0x00F0 (CPU register)
    /// c ---> 0xF000 (Opcode group)
    /// d ---> 0x000F (Opcode subgroup -- used in different contexts to 'n')
    /// kk --> 0x00FF (Integer)
    /// nnn -> 0x0FFF (Memory address)
    ///
    /// Three main opcode forms:
    ///
    /// - adding val to register (e.g. 0x73EE -> "add 238 [0xEE] to register 3")
    ///     - let (c, x, kk) = (0x7, 0x3, 0xEE);
    ///
    /// - jump to memory address (e.g. 0x1200 -> "jump to mem location 0x200")
    ///     - let (c, nnn) = (0x1, 0x200);
    ///
    /// - bitwise OR on two registers (e.g. 0x8231 -> "x = x | y")
    ///     - let (c, x, y, d) = (0x8, 0x2, 0x3, 0x1);
    ///
    fn decode(&self, opcode: &u16) -> (u8, u8, u8, u8) {
        (
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8,
        )
    }

    pub fn run(&mut self) {

        match self.decode(&self.read_opcode()) {
            (0x8, x, y, 0x4) => self.add_xy(x, y),
            _ => todo!("implement remaining opcodes!"),
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let lhs = self.reg[x as usize];
        let rhs = self.reg[y as usize];

        let (wrapped_val, overflow) = rhs.overflowing_add(lhs);
        self.reg[x as usize] = wrapped_val;

        // last register is used as a carry-flag
        // which indicates an operation has overflowed the u8 register size
        if overflow {
            self.reg[0xF] = 1;
        } else {
            self.reg[0xF] = 0;
        }

    }
}

pub fn addition_demo() {
    let mut cpu = CPU::new();

    // opcode:
    // > 8 -> the op involves two registers
    // > 0 -> signifies the first register
    // > 1 -> signifies the second register
    // > 4 -> indicates addition
    cpu.op = 0x8014;

    cpu.reg[0] = 5;
    cpu.reg[1] = 10;
}
