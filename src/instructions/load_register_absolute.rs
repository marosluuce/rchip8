use cpu::Cpu;
use instructions::instruction::Instruction;
use instructions::op::Op;

struct LoadRegisterAbsolute {
    register: usize,
    value: u8,
}

impl Op for LoadRegisterAbsolute {
    const MASK: u16 = 0x6FFF;
}

impl Instruction for LoadRegisterAbsolute {
    fn new(opcode: u16) -> LoadRegisterAbsolute {
        LoadRegisterAbsolute {
            register: ((opcode & 0x0F00) >> 8) as usize,
            value: (opcode & 0x00FF) as u8,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        Cpu {
            registers: {
                let mut registers = cpu.registers;
                registers[self.register] = self.value;
                registers
            },
            pc: cpu.pc + 2,
            ..cpu
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_a_value_into_register() {
        let instruction = LoadRegisterAbsolute::new(0x6672);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
        };

        assert_eq!(
            Cpu {
                pc: 6,
                registers: {
                    let mut registers = [0; 16];
                    registers[6] = 0x72;
                    registers
                },
                ..cpu
            },
            instruction.execute(cpu)
        );
    }
}
