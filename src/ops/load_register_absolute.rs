use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct LoadRegisterAbsolute {
    register: usize,
    value: u8,
}

impl Op for LoadRegisterAbsolute {
    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu.set_register(self.register, self.value).increment_pc()
    }
}

impl Matcher for LoadRegisterAbsolute {
    const MASK: u16 = 0x6FFF;

    fn new(opcode: u16) -> LoadRegisterAbsolute {
        LoadRegisterAbsolute {
            register: ((opcode & 0x0F00) >> 8) as usize,
            value: (opcode & 0x00FF) as u8,
        }
    }
}

impl fmt::Display for LoadRegisterAbsolute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD V{:X}, {:X}", self.register, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_a_value_into_register() {
        let op = LoadRegisterAbsolute::new(0x6672);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
        };

        assert_eq!(
            Cpu {
                pc: 6,
                ..cpu.set_register(6, 0x72)
            },
            op.execute(cpu)
        );
    }
}
