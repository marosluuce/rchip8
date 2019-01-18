use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

pub struct And {
    register1: usize,
    register2: usize,
}

impl Instruction for And {
    const MASK: u16 = 0x8FF2;

    fn new(opcode: u16) -> And {
        And {
            register1: ((opcode & 0x0F00) >> 8) as usize,
            register2: ((opcode & 0x00F0) >> 4) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu.update_register(self.register1, |x| x & cpu.registers[self.register2])
            .increment_pc()
    }
}

impl fmt::Display for And {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AND V{:X}, V{:X}", self.register1, self.register2)
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
                pc: 6,
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
