use cpu::Cpu;
use instructions::instruction::Instruction;
use instructions::op::Op;

struct SkipNotEqualsAbsolute {
    register: usize,
    value: u8,
}

impl Op for SkipNotEqualsAbsolute {
    const MASK: u16 = 0x4FFF;
}

impl Instruction for SkipNotEqualsAbsolute {
    fn new(opcode: u16) -> SkipNotEqualsAbsolute {
        SkipNotEqualsAbsolute {
            register: ((opcode & 0x0F00) >> 8) as usize,
            value: (opcode & 0x00FF) as u8,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        if cpu.registers[self.register] == self.value {
            Cpu {
                pc: cpu.pc + 1,
                ..cpu
            }
        } else {
            Cpu {
                pc: cpu.pc + 2,
                ..cpu
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executes_skip_when_register_does_not_equal_value() {
        let instruction = SkipNotEqualsAbsolute::new(0x4204);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
        };

        assert_eq!(
            Cpu {
                pc: 6,
                sp: 0,
                ..cpu
            },
            instruction.execute(cpu)
        );
    }

    #[test]
    fn does_not_skip_when_register_contains_value() {
        let instruction = SkipNotEqualsAbsolute::new(0x4204);
        let cpu = Cpu {
            pc: 4,
            registers: {
                let mut registers = [0; 16];
                registers[2] = 0x04;
                registers
            },
            ..Cpu::new()
        };

        assert_eq!(Cpu { pc: 5, ..cpu }, instruction.execute(cpu));
    }
}
