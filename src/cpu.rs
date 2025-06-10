/// A virtual CPU that implements a subset of CHIP-8 ops.
pub struct CPU {
    current_op: u16,
    registers: [u8; 2],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        self.current_op
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
        let opcode = self.read_opcode();

        match self.decode(&opcode) {
            (0x8, x, y, 0x4) => self.add_xy(x, y),
            _ => todo!("implement remaining opcodes!"),
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

pub fn addition_demo() {
    let mut cpu = CPU {
        current_op: 0,
        registers: [0, 2],
    };

    // opcode:
    // > 8 -> the op involves two registers
    // > 0 -> signifies the first register
    // > 1 -> signifies the second register
    // > 4 -> indicates addition
    cpu.current_op = 0x8014;

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
}
