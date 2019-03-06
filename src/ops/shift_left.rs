use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct ShiftLeft {
    register: usize,
}

impl Op for ShiftLeft {
    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu.set_register(0xF, cpu.registers[self.register] & 1)
            .update_register(self.register, |x| x * 2)
            .increment_pc()
    }
}

impl Matcher for ShiftLeft {
    const MASK: u16 = 0x8FFE;

    fn new(opcode: u16) -> ShiftLeft {
        ShiftLeft {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }
}

impl fmt::Display for ShiftLeft {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SHL V{:X}", self.register)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shifts_a_register_with_odd_value() {
        let op = ShiftLeft::new(0x873E);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
        }
        .set_register(7, 33);

        assert_eq!(
            Cpu {
                pc: 6,
                ..cpu.set_register(7, 66).set_register(0xF, 1)
            },
            op.execute(cpu)
        );
    }

    #[test]
    fn shifts_a_register_with_even_value() {
        let op = ShiftLeft::new(0x873E);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
        }
        .set_register(7, 34);

        assert_eq!(
            Cpu {
                pc: 6,
                ..cpu.set_register(7, 68).set_register(0xF, 0)
            },
            op.execute(cpu)
        );
    }
}
