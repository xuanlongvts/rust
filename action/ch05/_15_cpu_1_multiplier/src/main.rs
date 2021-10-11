const HALT: u8 = 0x0;
const ARITHMETIC_AND_LOGIC: u8 = 0x8;
const ADD_XY: u8 = 0x4;

#[derive(Debug)]
struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 10]
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        println!("op_byte1: {}", op_byte1);
        println!("op_byte2: {}", op_byte2);
        println!("======================");

        op_byte1 << 8 | op_byte2
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize]
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();

            self.position_in_memory += 2;

            let op_major = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let op_minor = ((opcode & 0x000F) >> 0) as u8;

            match (op_major, op_minor) {
                (HALT, HALT) => {return;},
                (ARITHMETIC_AND_LOGIC, ADD_XY) => self.add_xy(x, y),
                _ => unimplemented!("opcode {:04x}", opcode)
            }
        }
    }

}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 10],
        position_in_memory: 0,
    };
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    cpu.memory[0] = 0x80; // 1000 0000 ~ 2^7 = 128
    cpu.memory[1] = 0x14; // 0001 0100 ~ 2^4 + 2^2 = 20

    cpu.memory[2] = 0x80;
    cpu.memory[3] = 0x24;

    cpu.memory[4] = 0x80;
    cpu.memory[5] = 0x34;

    cpu.run();
    assert_eq!(cpu.registers[0], 35);
    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);

    println!("registers: {:?}", cpu.registers);
    println!("position_in_memory: {:?}", cpu.position_in_memory);
    println!("memory:: {:?}", cpu.memory);
}
