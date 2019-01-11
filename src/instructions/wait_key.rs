use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

struct WaitKey {
    register: usize,
}

impl Instruction for WaitKey {
    const MASK: u16 = 0xFF0A;

    fn new(opcode: u16) -> WaitKey {
        WaitKey {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}

impl fmt::Display for WaitKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD V{:X}, K", self.register)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_key_press_into_register() {}
}
