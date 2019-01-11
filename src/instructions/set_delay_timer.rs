use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

struct SetDelayTimer {
    register: usize,
}

impl Instruction for SetDelayTimer {
    const MASK: u16 = 0xFF15;

    fn new(opcode: u16) -> SetDelayTimer {
        SetDelayTimer {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}

impl fmt::Display for SetDelayTimer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD DT, V{:X}", self.register)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
