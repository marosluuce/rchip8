extern crate rchip8;

use rchip8::instruction::Instruction;
use rchip8::ops::add::Add;
use rchip8::ops::add_absolute::AddAbsolute;
use rchip8::ops::op::Matcher;

fn get_applicable_op(opcode: u16) -> Instruction {
    if Add::matches(opcode) {
        return Instruction {
            op: Box::new(Add::new(opcode)),
        };
    }

    if AddAbsolute::matches(opcode) {
        return Instruction {
            op: Box::new(AddAbsolute::new(opcode)),
        };
    }

    panic!("unknown op")
}

fn main() {
    println!("{}", get_applicable_op(0x8124));
    println!("{}", get_applicable_op(0x81F4));
    println!("{}", get_applicable_op(0x7124));
}
