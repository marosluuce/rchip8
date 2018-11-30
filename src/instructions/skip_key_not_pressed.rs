use cpu::Cpu;
use instructions::instruction::Instruction;
use instructions::op::Op;

struct SkipKeyNotPressed {
    register: usize,
}

impl Op for SkipKeyNotPressed {
    const MASK: u16 = 0xEFA1;
}

impl Instruction for SkipKeyNotPressed {
    fn new(opcode: u16) -> SkipKeyNotPressed {
        SkipKeyNotPressed {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
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
