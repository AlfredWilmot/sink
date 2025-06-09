
/// A virtual CPU that implements a subset of CHIP-8 ops.
pub struct CPU {
    current_op: u16,
    registers: [u8; 2],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        self.current_op
    }
    /// decode the passed CHIP-8 opcode into its components.
    fn decode(&self, opcode: &u16) -> (u8, u8, u8, u8) {
       return (
        ((opcode & 0xF000) >> 12) as u8,
        ((opcode & 0x0F00) >> 8) as u8,
        ((opcode & 0x00F0) >> 4) as u8,
        ((opcode & 0x000F) >> 0) as u8,
        )
    }

    pub fn run(&mut self) {
       let opcode = self.read_opcode();
       let (c, x, y, d) = self.decode(&opcode);
       match (c, x, y, d) {
           (0x8, _, _, 0x4) => self.add_xy(x, y),
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
        registers: [0,2],
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
