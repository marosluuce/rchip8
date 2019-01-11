use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

struct ShiftRight {
    register: usize,
}

impl Instruction for ShiftRight {
    const MASK: u16 = 0x8FF6;

    fn new(opcode: u16) -> ShiftRight {
        ShiftRight {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        Cpu {
            pc: cpu.pc + 1,
            registers: {
                let mut registers = cpu.registers;
                registers[0xF] = registers[self.register] & 1;
                registers[self.register] /= 2;
                registers
            },
            ..cpu
        }
    }
}

impl fmt::Display for ShiftRight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SHR V{:X}", self.register)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shifts_a_register_with_odd_value() {
        let instruction = ShiftRight::new(0x8736);
        let cpu = Cpu {
            pc: 4,
            registers: {
                let mut registers = [0; 16];
                registers[7] = 33;
                registers
            },
            ..Cpu::new()
        };

        assert_eq!(
            Cpu {
                pc: 5,
                registers: {
                    let mut registers = [0; 16];
                    registers[7] = 16;
                    registers[0xF] = 1;
                    registers
                },
                ..cpu
            },
            instruction.execute(cpu)
        );
    }

    #[test]
    fn shifts_a_register_with_even_value() {
        let instruction = ShiftRight::new(0x8736);
        let cpu = Cpu {
            pc: 4,
            registers: {
                let mut registers = [0; 16];
                registers[7] = 34;
                registers
            },
            ..Cpu::new()
        };

        assert_eq!(
            Cpu {
                pc: 5,
                registers: {
                    let mut registers = [0; 16];
                    registers[7] = 17;
                    registers[0xF] = 0;
                    registers
                },
                ..cpu
            },
            instruction.execute(cpu)
        );
    }
}
