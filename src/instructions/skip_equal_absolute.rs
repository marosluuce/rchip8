use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

pub struct SkipEqualsAbsolute {
    register: usize,
    value: u8,
}

impl Instruction for SkipEqualsAbsolute {
    const MASK: u16 = 0x3FFF;

    fn new(opcode: u16) -> SkipEqualsAbsolute {
        SkipEqualsAbsolute {
            register: ((opcode & 0x0F00) >> 8) as usize,
            value: (opcode & 0x00FF) as u8,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        if cpu.registers[self.register] == self.value {
            cpu.increment_pc().increment_pc()
        } else {
            cpu.increment_pc()
        }
    }
}

impl fmt::Display for SkipEqualsAbsolute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SE V{:X}, {:X}", self.register, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executes_skip_when_register_equals_value() {
        let instruction = SkipEqualsAbsolute::new(0x3204);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
        }
        .set_register(2, 4);

        assert_eq!(Cpu { pc: 8, ..cpu }, instruction.execute(cpu));
    }

    #[test]
    fn executes_does_not_skip() {
        let instruction = SkipEqualsAbsolute::new(0x3205);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
        };

        assert_eq!(Cpu { pc: 6, ..cpu }, instruction.execute(cpu));
    }
}
