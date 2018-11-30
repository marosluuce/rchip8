use cpu::Cpu;
use instructions::instruction::Instruction;
use instructions::op::Op;

struct Add {
    register1: usize,
    register2: usize,
}

impl Op for Add {
    const MASK: u16 = 0x8FF4;
}

impl Instruction for Add {
    fn new(opcode: u16) -> Add {
        Add {
            register1: ((opcode & 0x0F00) >> 8) as usize,
            register2: ((opcode & 0x00F0) >> 4) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        Cpu {
            pc: cpu.pc + 1,
            registers: {
                let mut registers = cpu.registers;
                let (value, overflow) =
                    registers[self.register1].overflowing_add(registers[self.register2]);

                registers[self.register1] = value;
                if overflow {
                    registers[0xF] = 1;
                } else {
                    registers[0xF] = 0;
                }
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
    fn adds_two_registers() {
        let instruction = Add::new(0x8244);
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
                    registers[2] = 4;
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
    fn adds_two_registers_with_overflow() {
        let instruction = Add::new(0x8244);
        let cpu = Cpu {
            pc: 4,
            registers: {
                let mut registers = [0; 16];
                registers[2] = 2;
                registers[4] = 0xFF;
                registers
            },
            ..Cpu::new()
        };

        assert_eq!(
            Cpu {
                pc: 5,
                registers: {
                    let mut registers = [0; 16];
                    registers[2] = 1;
                    registers[4] = 0xFF;
                    registers[0xF] = 1;
                    registers
                },
                ..cpu
            },
            instruction.execute(cpu)
        );
    }
}
