use cpu::Cpu;
use instructions::instruction::Instruction;
use instructions::op::Op;

struct WaitKey {
    register: usize,
}

impl Op for WaitKey {
    const MASK: u16 = 0xFF0A;
}

impl Instruction for WaitKey {
    fn new(opcode: u16) -> WaitKey {
        WaitKey {
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
    fn reads_a_key_press_into_register() {}
}
