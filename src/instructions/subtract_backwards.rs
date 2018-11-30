use cpu::Cpu;
use instructions::instruction::Instruction;
use instructions::op::Op;

struct SubtractBackwards {
    register1: usize,
    register2: usize,
}

impl Op for SubtractBackwards {
    const MASK: u16 = 0x8FF7;
}

impl Instruction for SubtractBackwards {
    fn new(opcode: u16) -> SubtractBackwards {
        SubtractBackwards {
            register1: ((opcode & 0x0F00) >> 8) as usize,
            register2: ((opcode & 0x00F0) >> 4) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        Cpu {
            pc: cpu.pc + 1,
            registers: {
                let mut registers = cpu.registers;
                if registers[self.register1] < registers[self.register2] {
                    registers[0xF] = 1;
                } else {
                    registers[0xF] = 0;
                }
                registers[self.register1] =
                    registers[self.register2].wrapping_sub(registers[self.register1]);
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
    fn subtracts_two_registers() {
        let instruction = SubtractBackwards::new(0x8247);
        let cpu = Cpu {
            pc: 4,
            registers: {
                let mut registers = [0; 16];
                registers[2] = 2;
                registers[4] = 2;
                registers[0xF] = 5;
                registers
            },
            ..Cpu::new()
        };

        assert_eq!(
            Cpu {
                pc: 5,
                registers: {
                    let mut registers = [0; 16];
                    registers[2] = 0;
                    registers[4] = 2;
                    registers[0xF] = 0;
                    registers
                },
                ..cpu
            },
            instruction.execute(cpu)
        );
    }

    #[test]
    fn subtracts_two_registers_with_no_borrow() {
        let instruction = SubtractBackwards::new(0x8245);
        let cpu = Cpu {
            pc: 4,
            registers: {
                let mut registers = [0; 16];
                registers[2] = 3;
                registers[4] = 5;
                registers
            },
            ..Cpu::new()
        };

        assert_eq!(
            Cpu {
                pc: 5,
                registers: {
                    let mut registers = [0; 16];
                    registers[2] = 2;
                    registers[4] = 5;
                    registers[0xF] = 1;
                    registers
                },
                ..cpu
            },
            instruction.execute(cpu)
        );
    }
}
