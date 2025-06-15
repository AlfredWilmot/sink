/// A virtual CPU that implements a subset of CHIP-8 ops.
pub struct CPU {
    pub reg: [u8; 16],    // 16 registers can be addressed by a single hex val (0-F)
    mem: [u8; 4096],  // 4K of RAM (0x1000): opcode written here drive the CPU FSM
    pc: usize,        // program counter: points to the current position in memory
    stack: [u16; 16], // support 16 nested function-calls before "stack overflow"
    sp: usize,        // stack pointer: points to the current position in the stack
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

/// indicates address space reserved for system memory
const RES_SYS_MEM: usize = 0x100; // 512 bytes

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

    /// write to the address space reserved for system opcodes
    pub fn write_system_mem(&mut self, ops: &[u8]) {
        if ops.len() as usize > RES_SYS_MEM {
            panic!("Cannot exceed system memory allocation!");
        }
        let start: usize = 0x000;
        let stop: usize = start + ops.len() as usize;
        self.mem[start..stop].copy_from_slice(&ops);
    }

    /// write to the address space reserved for program opcodes
    pub fn write_prog_mem(&mut self, ops: &[u8]) {
        let start: usize = RES_SYS_MEM;
        let stop: usize = start + ops.len() as usize;
        self.mem[start..stop].copy_from_slice(&ops);
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

    /// add a new entry to the call-stack
    pub fn call(&mut self, addr: u16) {
        // cannot reference beyond the address space allocated to the stack!
        if self.sp > self.stack.len() {
            panic!("Stack Overflow");
        }

        // keep track of where the program counter has been pointing:
        // > update the value of the call-stack currently referenced by the stack pointer
        // > increment the stack pinter in preparation for the next call
        // > update the program counter with the address that was called
        self.stack[self.sp] = self.pc as u16;
        self.sp += 1;
        self.pc = addr as usize;
    }

    /// move down the call-stack
    pub fn ret(&mut self) {
        if self.sp == 0 {
            panic!("Stack Underflow!")
        }
        self.sp -= 1;
        self.pc = self.stack[self.sp] as usize;
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.pc += 2; // each mem blk is u8 and can hold half a u16 instruction,
            // so shift the program-counter to the next instruction that's
            // sitting two blocks away from the current instruction

            let nnn = opcode & 0x0FFF;
            //let kk = (opcode & 0x00FF) as u8;

            match self.decode(&opcode) {
                (0, 0, 0, 0) => return,
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
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
    let expected_sum = summands.iter().sum(); // 5 + 10 + 10 + 10 = 35

    // load registers with summands
    for (idx, val) in summands.iter().enumerate() {
        cpu.reg[idx] = *val;
    }

    (cpu.mem[0], cpu.mem[1]) = (0x80, 0x14); // 0x8014 (8: two registers [0 & 1], 4: addition)
    (cpu.mem[2], cpu.mem[3]) = (0x80, 0x24); // 0x8024 (8: two registers [0 & 2], 4: addition)
    (cpu.mem[4], cpu.mem[5]) = (0x80, 0x34); // 0x8034 (8: two registers [0 & 3], 4: addition)
                                             //
    cpu.run();
    assert_eq!(cpu.reg[0], expected_sum);
}

#[test]
pub fn test_call_and_return() {
    // instantiate a virtual CPU
    let mut cpu = CPU::new();

    // define some values for testing
    let args = [5, 10];
    let expected_sum = args[0] + args[1] * 2 + args[1] * 2; // 5 + (10 * 2 ) * 2

    // load the values into the registiers
    for (idx, val) in args.iter().enumerate() {
        cpu.reg[idx] = *val;
    }

    // call the function loaded at 0x100 twice
    let call_func_twice: [u8; 6] = [0x21, 0x00, 0x21, 0x00, 0x00, 0x00];
    cpu.write_system_mem(&call_func_twice);

    // define a function composed of opcodes
    let add_twice_func: [u8; 6] = [
        0x80, 0x14, // ADD reg 1 to reg 0
        0x80, 0x14, // --||--
        0x00, 0xEE, // RETURN
    ];
    cpu.write_prog_mem(&add_twice_func);

    cpu.run();
    assert_eq!(cpu.reg[0], expected_sum);
}
