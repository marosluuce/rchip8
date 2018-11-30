use cpu::Cpu;
use instructions::instruction::Instruction;
use instructions::op::Op;

struct SkipKeyPressed {
    register: usize,
}

impl Op for SkipKeyPressed {
    const MASK: u16 = 0xEF9E;
}

impl Instruction for SkipKeyPressed {
    fn new(opcode: u16) -> SkipKeyPressed {
        SkipKeyPressed {
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
    fn skips_when_key_pressed() {}

    #[test]
    fn does_not_skip_when_key_unpressed() {}
}
