use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct Add {
    register1: usize,
    register2: usize,
}

impl Op for Add {
    fn execute(&self, cpu: Cpu) -> Cpu {
        let (value, overflow) =
            cpu.registers[self.register1].overflowing_add(cpu.registers[self.register2]);

        cpu.set_register(self.register1, value)
            .set_register(0xF, overflow as u8)
            .increment_pc()
    }
}

impl fmt::Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ADD V{:X}, V{:X}", self.register1, self.register2)
    }
}

impl Matcher for Add {
    const MASK: u16 = 0x8FF4;

    fn new(opcode: u16) -> Add {
        Add {
            register1: ((opcode & 0x0F00) >> 8) as usize,
            register2: ((opcode & 0x00F0) >> 4) as usize,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_registers() {
        let op = Add::new(0x8244);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
                .set_register(2, 2)
                .set_register(4, 2)
                .set_register(0xF, 5)
        };

        assert_eq!(
            Cpu {
                pc: 6,
                ..Cpu::new()
                    .set_register(2, 4)
                    .set_register(4, 2)
                    .set_register(0xF, 0)
            },
            op.execute(cpu)
        );
    }

    #[test]
    fn adds_two_registers_with_overflow() {
        let op = Add::new(0x8244);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new().set_register(2, 2).set_register(4, 0xFF)
        };

        assert_eq!(
            Cpu {
                pc: 6,
                ..Cpu::new()
                    .set_register(2, 1)
                    .set_register(4, 0xFF)
                    .set_register(0xF, 1)
            },
            op.execute(cpu)
        );
    }
}
