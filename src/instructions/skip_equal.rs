use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

pub struct SkipEqual {
    register1: usize,
    register2: usize,
}

impl Instruction for SkipEqual {
    const MASK: u16 = 0x5FF0;

    fn new(opcode: u16) -> SkipEqual {
        SkipEqual {
            register1: ((opcode & 0x0F00) >> 8) as usize,
            register2: ((opcode & 0x00F0) >> 4) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        if cpu.registers[self.register1] == cpu.registers[self.register2] {
            cpu.increment_pc().increment_pc()
        } else {
            cpu.increment_pc()
        }
    }
}

impl fmt::Display for SkipEqual {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SE V{:X}, V{:X}", self.register1, self.register2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executes_skip_when_registers_equal() {
        let instruction = SkipEqual::new(0x5280);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
        }
        .set_register(2, 4)
        .set_register(8, 4);

        assert_eq!(
            Cpu {
                pc: 8,
                sp: 0,
                ..cpu
            },
            instruction.execute(cpu)
        );
    }

    #[test]
    fn executes_does_not_skip() {
        let instruction = SkipEqual::new(0x5260);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
        }
        .set_register(2, 4)
        .set_register(6, 5);

        assert_eq!(Cpu { pc: 6, ..cpu }, instruction.execute(cpu));
    }
}
