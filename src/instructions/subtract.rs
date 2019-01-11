use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

struct Subtract {
    register1: usize,
    register2: usize,
}

impl Instruction for Subtract {
    const MASK: u16 = 0x8FF5;

    fn new(opcode: u16) -> Subtract {
        Subtract {
            register1: ((opcode & 0x0F00) >> 8) as usize,
            register2: ((opcode & 0x00F0) >> 4) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        Cpu {
            pc: cpu.pc + 1,
            registers: {
                let mut registers = cpu.registers;
                if registers[self.register1] > registers[self.register2] {
                    registers[0xF] = 1;
                } else {
                    registers[0xF] = 0;
                }
                registers[self.register1] =
                    registers[self.register1].wrapping_sub(registers[self.register2]);
                registers
            },
            ..cpu
        }
    }
}

impl fmt::Display for Subtract {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SUB V{:X}, V{:X}", self.register1, self.register2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtracts_two_registers() {
        let instruction = Subtract::new(0x8245);
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
        let instruction = Subtract::new(0x8245);
        let cpu = Cpu {
            pc: 4,
            registers: {
                let mut registers = [0; 16];
                registers[2] = 5;
                registers[4] = 3;
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
                    registers[4] = 3;
                    registers[0xF] = 1;
                    registers
                },
                ..cpu
            },
            instruction.execute(cpu)
        );
    }
}
