use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

pub struct JumpAbsolute {
    address: u16,
}

impl Instruction for JumpAbsolute {
    const MASK: u16 = 0x1FFF;

    fn new(opcode: u16) -> JumpAbsolute {
        JumpAbsolute {
            address: opcode & 0x0FFF,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        Cpu {
            pc: self.address,
            ..cpu
        }
    }
}

impl fmt::Display for JumpAbsolute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JP {:X}", self.address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executes_jump_absolute() {
        let instruction = JumpAbsolute::new(0x1123);
        let cpu = Cpu::new();

        assert_eq!(Cpu { pc: 0x123, ..cpu }, instruction.execute(cpu));
    }
}
