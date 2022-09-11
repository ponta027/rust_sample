// list 5-18
const REG_SIZE: usize = 16;
const MEM_SIZE: usize = 0x1000;
struct CPU {
    //    current_operation: u16,
    //    registers: [u8; 2],
    registers: [u8; REG_SIZE],
    position_in_memory: usize,
    memory: [u8; MEM_SIZE],
}

impl CPU {
    fn read_opecode(&self) -> u16 {
        //list5-24
        let p = self.position_in_memory;
        //        self.current_operation
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        op_byte1 << 8 | op_byte2 << 0
    }
    fn run(&mut self) {
        loop {
            let opcode = self.read_opecode();
            self.position_in_memory += 2;
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            println!("{},{},{},{}", c, x, y, d);
            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0x8, _, _, 0x4) => self.add_x_y(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }
    fn add_x_y(&mut self, x: u8, y: u8) {
        self.dump();
        println!("x={},y={}", x, y);
        //list5-25
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);

        self.registers[x as usize] = val;
        if overflow {
            self.registers[0xF] = 0;
        } else {
            self.registers[0xF] = 1;
        }

        self.dump();
    }
    fn dump(&self) -> () {
        /*
        println!(
            "0x{:0x}:{:?}",          //
            self.position_in_memory, //
            self.registers
        );
        */
        ()
    }
}
fn main() {
    println!("START");
    // list 5-19
    // CPUを初期化
    let mut cpu = CPU {
        memory: [0; MEM_SIZE],
        //current_operation: 0,
        registers: [0; REG_SIZE],
        position_in_memory: 0,
    };
    cpu.dump();
    //    cpu.current_operation = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;
    cpu.dump();

    let mem = &mut cpu.memory;
    mem[0] = 0x80;
    mem[1] = 0x14;
    mem[2] = 0x80;
    mem[3] = 0x24;
    mem[4] = 0x80;
    mem[5] = 0x34;

    cpu.run();
    cpu.dump();

    assert_eq!(cpu.registers[0], 35);
    println!("END");
}
