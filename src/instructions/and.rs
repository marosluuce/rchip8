use cpu::Cpu;
use instructions::instruction::Instruction;
use instructions::op::Op;

struct And {
    register1: usize,
    register2: usize,
}

impl Op for And {
    const MASK: u16 = 0x8FF2;
}

impl Instruction for And {
    fn new(opcode: u16) -> And {
        And {
            register1: ((opcode & 0x0F00) >> 8) as usize,
            register2: ((opcode & 0x00F0) >> 4) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        Cpu {
            pc: cpu.pc + 1,
            registers: {
                let mut registers = cpu.registers;
                registers[self.register1] &= registers[self.register2];
                registers
            },
            ..cpu
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_register2_into_register1() {
        let instruction = And::new(0x8281);
        let cpu = Cpu {
            pc: 4,
            registers: {
                let mut registers = [0; 16];
                registers[2] = 0x04;
                registers[8] = 0xFF;
                registers
            },
            ..Cpu::new()
        };

        assert_eq!(
            Cpu {
                pc: 5,
                registers: {
                    let mut registers = [0; 16];
                    registers[2] = 0x04;
                    registers[8] = 0xFF;
                    registers
                },
                ..cpu
            },
            instruction.execute(cpu)
        );
    }
}
