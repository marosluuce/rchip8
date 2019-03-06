use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct ShiftRight {
    register: usize,
}

impl Op for ShiftRight {
    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu.set_register(0xF, cpu.registers[self.register] & 1)
            .update_register(self.register, |x| x / 2)
            .increment_pc()
    }
}

impl Matcher for ShiftRight {
    const MASK: u16 = 0x8FF6;

    fn new(opcode: u16) -> ShiftRight {
        ShiftRight {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }
}

impl fmt::Display for ShiftRight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SHR V{:X}", self.register)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shifts_a_register_with_odd_value() {
        let op = ShiftRight::new(0x8736);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
        }
        .set_register(7, 33);

        assert_eq!(
            Cpu {
                pc: 6,
                ..cpu.set_register(7, 16).set_register(0xF, 1)
            },
            op.execute(cpu)
        );
    }

    #[test]
    fn shifts_a_register_with_even_value() {
        let op = ShiftRight::new(0x8736);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
        }
        .set_register(7, 34);

        assert_eq!(
            Cpu {
                pc: 6,
                ..cpu.set_register(7, 17).set_register(0xF, 0)
            },
            op.execute(cpu)
        );
    }
}
