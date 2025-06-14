/// A virtual CPU that implements a subset of CHIP-8 ops.
pub struct CPU {
    reg: [u8; 16],    // 16 registers can be addressed by a single hex val (0-F)
    pc: usize,        // program counter: points to the current position in memory
    mem: [u8; 4096],  // 4K of RAM (0x1000): opcode written here drive the CPU FSM
    stack: [u16; 16], // support 16 nested function-calls before "stack overflow"
    sp: usize,        // stack pointer: points to the current position in the stack
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    /// instantiates a default CPU
    pub fn new() -> CPU {
        CPU {
            reg: [0; 16],
            pc: 0,
            mem: [0; 4096],
            stack: [0; 16],
            sp: 0,
        }
    }

    /// read in the current operation referenced by the program_counter
    fn read_opcode(&self) -> u16 {
        let op_byte1 = self.mem[self.pc] as u16; // 0b00000000XXXXXXXX
        let op_byte2 = self.mem[self.pc + 1] as u16; // 0b00000000YYYYYYYY

        (op_byte1 << 8) | op_byte2 // 0bXXXXXXXXYYYYYYYY
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
            ((opcode & 0x000F) >> 0) as u8,
        )
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.pc += 2; // each mem blk is u8 and can hold half a u16 instruction,
            // so shift the program-counter to the next instruction that's
            // sitting two blocks away from the current instruction
            match self.decode(&opcode) {
                (0, 0, 0, 0) => return,
                (0x8, x, y, 0x4) => self.add_xy(x, y),
                _ => todo!("implement remaining opcodes!"),
            }
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

#[test]
/// validating expected CPU behaviour using the addition opcode
/// NOTE: test executable (generated with 'cargo test') is stored under 'target/debug/deps/'
pub fn test_addition() {
    let mut cpu = CPU::new();

    // define some values for testing
    let summands = [5, 10, 10, 10];
    let expected_sum = summands.iter().sum();

    // load registers with summands
    for (idx, val) in summands.iter().enumerate() {
        cpu.reg[idx] = *val;
    }

    (cpu.mem[0], cpu.mem[1]) = (0x80, 0x14); // 0x8014 (8: two registers [0 & 1], 4: addition)
    (cpu.mem[2], cpu.mem[3]) = (0x80, 0x24); // 0x8024 (8: two registers [0 & 2], 4: addition)
    (cpu.mem[4], cpu.mem[5]) = (0x80, 0x34); // 0x8034 (8: two registers [0 & 3], 4: addition)

    cpu.run();
    assert_eq!(cpu.reg[0], expected_sum);
}

#[test]
pub fn test_call_and_return() {

    // instantiate a virtual CPU
    let mut cpu = CPU::new();

    // define a function composed of opcodes
    let add_twice: [u8; 6] = [
        0x80, 0x14,  // ADD reg 1 to reg 0
        0x80, 0x14,  // --||--
        0xEE, 0x00,  // RETURN
    ];

    // load the function into memory
    cpu.mem[0x100..0x106].copy_from_slice(&add_twice);
}
