use cpu::Cpu;
use instructions::instruction::Instruction;
use instructions::op::Op;

struct SkipNotEqual {
    register1: usize,
    register2: usize,
}

impl Op for SkipNotEqual {
    const MASK: u16 = 0x9FF0;
}

impl Instruction for SkipNotEqual {
    fn new(opcode: u16) -> SkipNotEqual {
        SkipNotEqual {
            register1: ((opcode & 0x0F00) >> 8) as usize,
            register2: ((opcode & 0x00F0) >> 4) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        if cpu.registers[self.register1] == cpu.registers[self.register2] {
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
    fn executes_skip_when_registers_not_equal() {
        let instruction = SkipNotEqual::new(0x9280);
        let cpu = Cpu {
            pc: 4,
            registers: {
                let mut registers = [0; 16];
                registers[2] = 0;
                registers[8] = 1;
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
        let instruction = SkipNotEqual::new(0x9260);
        let cpu = Cpu {
            pc: 4,
            registers: {
                let mut registers = [0; 16];
                registers[2] = 1;
                registers[6] = 1;
                registers
            },
            ..Cpu::new()
        };

        assert_eq!(Cpu { pc: 5, ..cpu }, instruction.execute(cpu));
    }
}
