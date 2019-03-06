use cpu::Cpu;
use std::fmt;

pub trait Op: fmt::Display {
    fn execute(&self, cpu: Cpu) -> Cpu;
}

pub trait Matcher {
    const MASK: u16;

    fn matches(opcode: u16) -> bool {
        opcode & Self::MASK == opcode
    }

    fn new(opcode: u16) -> Self;
}
