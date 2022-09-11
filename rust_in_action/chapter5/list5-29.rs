// remain
// https://github.com/rust-in-action/code/blob/1st-edition/ch5/ch5-cpu4/src/main.rs
// https://en.wikipedia.org/wiki/CHIP-8
/*
 * type | opcode
 * -----|---------
 * ret  | 0x00EE
 * jmp  | 0x1**
 * call | 0x2**
 * if   | 0x3**
 * if not | 0x4**
 * if variable | 0x5**
 * ld   | 0x6**
 * add  | 0x7**
 * ld   | 0x8**0
 * or_xy | 0x8**1
 * and_xy | 0x8**2
 * xor_xy | 0x8**3
 * add_xy | 0x8**4
 *
 */
const REG_SIZE: usize = 16;
const MEM_SIZE: usize = 0x1000;
const STACK_SIZE: usize = 16;

struct CPU {
    /* list 5-27  add stack pointer */
    registers: [u8; REG_SIZE],
    position_in_memory: usize,
    stack: [u16; STACK_SIZE],
    memory: [u8; MEM_SIZE],

    //list5-28
    stack_pointer: usize,
}

impl CPU {
    fn read_opecode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        op_byte1 << 8 | op_byte2 << 0
    }
    fn run(&mut self) {
        loop {
            self.dump();
            let opcode = self.read_opecode();
            self.position_in_memory += 2;
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;
            let nn = ((opcode & 0x00FF) >> 0) as u8;

            //            println!("{},{},{},{}", c, x, y, d);

            let nnn = opcode & 0xFFF;
            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0x0, 0x0, 0xE, 0xE) => self.ret(),
                (0x1, _, _, _) => self.jmp(nnn),
                (0x2, _, _, _) => self.call(nnn),
                (0x3, _, _, _) => self.se(x, nn),
                (0x4, _, _, _) => self.sne(x, nn),
                (0x5, _, _, _) => self.se(x, y),
                (0x6, _, _, _) => self.ld(x, nn),
                (0x7, _, _, _) => self.add(x, nn),
                (0x8, _, _, 0x0) => self.ld(x, self.registers[y as usize]),
                (0x8, _, _, 0x1) => self.or_xy(x, y),
                (0x8, _, _, 0x2) => self.and_xy(x, y),
                (0x8, _, _, 0x3) => self.xor_xy(x, y),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn and_xy(&mut self, x: u8, y: u8) {
        let _x = self.registers[x as usize];
        let _y = self.registers[y as usize];
        self.registers[x as usize] = _x & _y;
    }
    fn or_xy(&mut self, x: u8, y: u8) {
        let _x = self.registers[x as usize];
        let _y = self.registers[y as usize];
        self.registers[x as usize] = _x | _y;
    }
    fn xor_xy(&mut self, x: u8, y: u8) {
        let _x = self.registers[x as usize];
        let _y = self.registers[y as usize];
        self.registers[x as usize] = _x ^ _y;
    }

    fn ld(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] = kk;
    }
    fn sne(&mut self, vx: u8, nn: u8) {
        if vx != nn {
            self.position_in_memory += 2;
        }
    }

    fn se(&mut self, vx: u8, nn: u8) {
        if vx == nn {
            self.position_in_memory += 2;
        }
    }

    fn jmp(&mut self, addr: u16) {
        println!("jmp:{}", addr);
        self.position_in_memory = addr as usize;

        //
    }
    fn add(&mut self, x: u8, kk: u8) {
        self.registers[x as usize] += kk;
    }
    fn add_xy(&mut self, x: u8, y: u8) {
        //        self.dump();
        //        println!("x={},y={}", x, y);
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);

        self.registers[x as usize] = val;
        if overflow {
            self.registers[0xF] = 0;
        } else {
            self.registers[0xF] = 1;
        }

        //        self.dump();
    }
    fn dump(&self) -> () {
        println!(
            "PC:0x{:04x}:STACK{:?}", //
            self.position_in_memory, //
            self.stack[0],
        );
        ()
    }
    // list5-28
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;
        if sp > stack.len() {
            panic!("Stack overflow");
        }
        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }
        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.position_in_memory = call_addr as usize;
        println!("ret:position:0x{:04x}", self.position_in_memory);
    }
}
fn main() {
    println!("START");

    // list5-28
    /*
    let add_twice: [u16; 3] = [
        0x8014, //
        0x8014, //
        0x00EE, //
    ];
    */

    let mut memory: [u8; MEM_SIZE] = [0; MEM_SIZE];
    let mem = &mut memory;

    let add_twice2 = [
        0x80, 0x14, //
        0x80, 0x14, //
        0x00, 0xEE, //
    ];
    mem[0x100..0x106].copy_from_slice(&add_twice2);

    //    println!("{:?}", &mem[0x100..0x106]);
    // list5-28

    let mut cpu = CPU {
        memory: [0; MEM_SIZE],
        registers: [0; REG_SIZE],
        position_in_memory: 0,
        stack: [0; STACK_SIZE],
        stack_pointer: 0,
    };
    //    cpu.dump();
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;
    //    cpu.dump();

    let mem = &mut cpu.memory;
    let assemble = [
        0x21, 0x00, // call
        0x41, 0x01, //
        0x21, 0x10, // call
        0x31, 0x01, //
        0x21, 0x00, //call
        0x60, 0x01, // ld
        0x70, 0x2c, // add
        0x00, 0x00, //END
    ];

    mem[0..16].copy_from_slice(&assemble);

    mem[0x100] = 0x80;
    mem[0x101] = 0x14;
    mem[0x102] = 0x80;
    mem[0x103] = 0x24;
    mem[0x104] = 0x00;
    mem[0x105] = 0xEE;

    mem[0x110] = 0x21;
    mem[0x111] = 0x00;
    mem[0x112] = 0x21;
    mem[0x113] = 0x00;
    mem[0x114] = 0x00;
    mem[0x115] = 0xEE;

    cpu.run();
    //    cpu.dump();

    assert_eq!(cpu.registers[0], 45);
    println!("END");
}
