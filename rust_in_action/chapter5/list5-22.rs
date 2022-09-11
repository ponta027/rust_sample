// list 5-18
struct CPU {
    current_operation: u16,
    registers: [u8; 2],
}

impl CPU {
    fn read_opecode(&self) -> u16 {
        self.current_operation
    }
    fn run(&mut self) {
        //loop
        {
            let opcode = self.read_opecode();
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                (0x8, _, _, 0x4) => self.add_x_y(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }
    fn add_x_y(&mut self, x: u8, y: u8) {
        println!("{},{}", x, y);
        self.dump();
        self.registers[x as usize] += self.registers[y as usize];
    }
    fn dump(&self) -> () {
        println!(
            "0x{:0x}:{:?}",         //
            self.current_operation, //
            self.registers
        );
        ()
    }
}
fn main() {
    println!("START");
    // list 5-19
    // CPUを初期化
    let mut cpu = CPU {
        current_operation: 0,
        registers: [0; 2],
    };
    cpu.dump();
    cpu.current_operation = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.dump();
    // レジスタ群にu8の値をロードする。

    // オペコードをロード
    // 演算する
    cpu.run();
    cpu.dump();
    assert_eq!(cpu.registers[0], 15);
    assert_eq!(cpu.registers[1], 10);
    println!("END");
    //
}
