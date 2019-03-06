use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct LoadRegister {
    register1: usize,
    register2: usize,
}

impl Op for LoadRegister {
    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu.set_register(self.register1, cpu.registers[self.register2])
            .increment_pc()
    }
}

impl Matcher for LoadRegister {
    const MASK: u16 = 0x8FF0;

    fn new(opcode: u16) -> LoadRegister {
        LoadRegister {
            register1: ((opcode & 0x0F00) >> 8) as usize,
            register2: ((opcode & 0x00F0) >> 4) as usize,
        }
    }
}

impl fmt::Display for LoadRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD V{:X}, V{:X}", self.register1, self.register2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_register2_into_register1() {
        let op = LoadRegister::new(0x8280);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new().set_register(2, 4).set_register(8, 0xF5)
        };

        assert_eq!(
            Cpu {
                pc: 6,
                ..Cpu::new().set_register(2, 0xF5).set_register(8, 0xF5)
            },
            op.execute(cpu)
        );
    }
}
