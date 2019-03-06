use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct SkipNotEqualsAbsolute {
    register: usize,
    value: u8,
}

impl Op for SkipNotEqualsAbsolute {
    fn execute(&self, cpu: Cpu) -> Cpu {
        if cpu.registers[self.register] == self.value {
            cpu.increment_pc()
        } else {
            cpu.increment_pc().increment_pc()
        }
    }
}

impl Matcher for SkipNotEqualsAbsolute {
    const MASK: u16 = 0x4FFF;

    fn new(opcode: u16) -> SkipNotEqualsAbsolute {
        SkipNotEqualsAbsolute {
            register: ((opcode & 0x0F00) >> 8) as usize,
            value: (opcode & 0x00FF) as u8,
        }
    }
}

impl fmt::Display for SkipNotEqualsAbsolute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SNE V{:X}, {:X}", self.register, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executes_skip_when_register_does_not_equal_value() {
        let op = SkipNotEqualsAbsolute::new(0x4204);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
        };

        assert_eq!(Cpu { pc: 8, ..cpu }, op.execute(cpu));
    }

    #[test]
    fn does_not_skip_when_register_contains_value() {
        let op = SkipNotEqualsAbsolute::new(0x4204);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
        }
        .set_register(2, 4);

        assert_eq!(Cpu { pc: 6, ..cpu }, op.execute(cpu));
    }
}
