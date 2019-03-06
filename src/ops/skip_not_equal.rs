use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct SkipNotEqual {
    register1: usize,
    register2: usize,
}

impl Op for SkipNotEqual {
    fn execute(&self, cpu: Cpu) -> Cpu {
        if cpu.registers[self.register1] == cpu.registers[self.register2] {
            cpu.increment_pc()
        } else {
            cpu.increment_pc().increment_pc()
        }
    }
}

impl Matcher for SkipNotEqual {
    const MASK: u16 = 0x9FF0;

    fn new(opcode: u16) -> SkipNotEqual {
        SkipNotEqual {
            register1: ((opcode & 0x0F00) >> 8) as usize,
            register2: ((opcode & 0x00F0) >> 4) as usize,
        }
    }
}

impl fmt::Display for SkipNotEqual {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SNE V{:X}, V{:X}", self.register1, self.register2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executes_skip_when_registers_not_equal() {
        let op = SkipNotEqual::new(0x9280);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
        }
        .set_register(2, 0)
        .set_register(8, 1);

        assert_eq!(Cpu { pc: 8, ..cpu }, op.execute(cpu));
    }

    #[test]
    fn executes_does_not_skip() {
        let op = SkipNotEqual::new(0x9260);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
        }
        .set_register(2, 1)
        .set_register(6, 1);

        assert_eq!(Cpu { pc: 6, ..cpu }, op.execute(cpu));
    }
}
