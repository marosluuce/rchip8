use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct Xor {
    register1: usize,
    register2: usize,
}

impl Op for Xor {
    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu.update_register(self.register1, |x| x ^ cpu.registers[self.register2])
            .increment_pc()
    }
}

impl Matcher for Xor {
    const MASK: u16 = 0x8FF3;

    fn new(opcode: u16) -> Xor {
        Xor {
            register1: ((opcode & 0x0F00) >> 8) as usize,
            register2: ((opcode & 0x00F0) >> 4) as usize,
        }
    }
}

impl fmt::Display for Xor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XOR V{:X}, V{:X}", self.register1, self.register2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_register2_into_register1() {
        let op = Xor::new(0x8281);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new().set_register(2, 0xF0).set_register(8, 0xFF)
        };

        assert_eq!(
            Cpu {
                pc: 6,
                ..Cpu::new().set_register(2, 0x0F).set_register(8, 0xFF)
            },
            op.execute(cpu)
        );
    }
}
