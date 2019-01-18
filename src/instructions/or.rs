use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

pub struct Or {
    register1: usize,
    register2: usize,
}

impl Instruction for Or {
    const MASK: u16 = 0x8FF1;

    fn new(opcode: u16) -> Or {
        Or {
            register1: ((opcode & 0x0F00) >> 8) as usize,
            register2: ((opcode & 0x00F0) >> 4) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu.update_register(self.register1, |x| x | cpu.registers[self.register2])
            .increment_pc()
    }
}

impl fmt::Display for Or {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OR V{:X}, V{:X}", self.register1, self.register2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_register2_into_register1() {
        let instruction = Or::new(0x8281);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new().set_register(2, 0x04).set_register(8, 0xFF)
        };

        assert_eq!(
            Cpu {
                pc: 6,
                ..Cpu::new().set_register(2, 0xFF).set_register(8, 0xFF)
            },
            instruction.execute(cpu)
        );
    }
}
