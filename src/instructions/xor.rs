use cpu::Cpu;
use instructions::instruction::Instruction;
use instructions::op::Op;

struct Xor {
    register1: usize,
    register2: usize,
}

impl Op for Xor {
    const MASK: u16 = 0x8FF3;
}

impl Instruction for Xor {
    fn new(opcode: u16) -> Xor {
        Xor {
            register1: ((opcode & 0x0F00) >> 8) as usize,
            register2: ((opcode & 0x00F0) >> 4) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        Cpu {
            pc: cpu.pc + 1,
            registers: {
                let mut registers = cpu.registers;
                registers[self.register1] ^= registers[self.register2];
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
        let instruction = Xor::new(0x8281);
        let cpu = Cpu {
            pc: 4,
            registers: {
                let mut registers = [0; 16];
                registers[2] = 0xF0;
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
                    registers[2] = 0x0F;
                    registers[8] = 0xFF;
                    registers
                },
                ..cpu
            },
            instruction.execute(cpu)
        );
    }
}
