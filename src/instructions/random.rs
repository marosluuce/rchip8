use cpu::Cpu;
use instructions::instruction::Instruction;
use instructions::op::Op;

struct SkipEqual {
    register1: usize,
    register2: usize,
}

impl Op for SkipEqual {
    const MASK: u16 = 0x5FF0;
}

impl Instruction for SkipEqual {
    fn new(opcode: u16) -> SkipEqual {
        SkipEqual {
            register1: ((opcode & 0x0F00) >> 8) as usize,
            register2: ((opcode & 0x00F0) >> 4) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        if cpu.registers[self.register1] == cpu.registers[self.register2] {
            Cpu {
                pc: cpu.pc + 2,
                ..cpu
            }
        } else {
            Cpu {
                pc: cpu.pc + 1,
                ..cpu
            }
        }
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
            registers: {
                let mut registers = [0; 16];
                registers[2] = 0x04;
                registers[8] = 0x04;
                registers
            },
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
    fn executes_does_not_skip() {
        let instruction = SkipEqual::new(0x5260);
        let cpu = Cpu {
            pc: 4,
            registers: {
                let mut registers = [0; 16];
                registers[2] = 0x04;
                registers[6] = 0x05;
                registers
            },
            ..Cpu::new()
        };

        assert_eq!(Cpu { pc: 5, ..cpu }, instruction.execute(cpu));
    }
}
