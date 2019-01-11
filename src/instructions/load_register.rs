use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

struct LoadRegister {
    register1: usize,
    register2: usize,
}

impl Instruction for LoadRegister {
    const MASK: u16 = 0x8FF0;

    fn new(opcode: u16) -> LoadRegister {
        LoadRegister {
            register1: ((opcode & 0x0F00) >> 8) as usize,
            register2: ((opcode & 0x00F0) >> 4) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        Cpu {
            pc: cpu.pc + 1,
            registers: {
                let mut registers = cpu.registers;
                registers[self.register1] = registers[self.register2];
                registers
            },
            ..cpu
        }
    }
}

impl fmt::Display for LoadRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD V{:X}, V{:X}", self.register1, self.register2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_register2_into_register1() {
        let instruction = LoadRegister::new(0x8280);
        let cpu = Cpu {
            pc: 4,
            registers: {
                let mut registers = [0; 16];
                registers[2] = 0x04;
                registers[8] = 0xF5;
                registers
            },
            ..Cpu::new()
        };

        assert_eq!(
            Cpu {
                pc: 5,
                registers: {
                    let mut registers = [0; 16];
                    registers[2] = 0xF5;
                    registers[8] = 0xF5;
                    registers
                },
                ..cpu
            },
            instruction.execute(cpu)
        );
    }
}
