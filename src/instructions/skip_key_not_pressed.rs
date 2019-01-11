use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

struct SkipKeyNotPressed {
    register: usize,
}

impl Instruction for SkipKeyNotPressed {
    const MASK: u16 = 0xEFA1;

    fn new(opcode: u16) -> SkipKeyNotPressed {
        SkipKeyNotPressed {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}

impl fmt::Display for SkipKeyNotPressed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SKNP V{:X}", self.register)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_when_key_not_pressed() {}

    #[test]
    fn does_not_skip_when_key_pressed() {}
}
