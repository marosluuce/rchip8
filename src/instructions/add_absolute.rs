use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

pub struct AddAbsolute {
    register: usize,
    value: u8,
}

impl Instruction for AddAbsolute {
    const MASK: u16 = 0x7FFF;

    fn new(opcode: u16) -> AddAbsolute {
        AddAbsolute {
            register: ((opcode & 0x0F00) >> 8) as usize,
            value: (opcode & 0x00FF) as u8,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu.update_register(self.register, |x| x + self.value)
            .increment_pc()
    }
}

impl fmt::Display for AddAbsolute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ADD V{:X}, {:X}", self.register, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_to_a_register() {
        let instruction = AddAbsolute::new(0x72F0);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new().set_register(2, 4)
        };

        assert_eq!(
            Cpu {
                pc: 6,
                ..cpu.set_register(2, 0xF4)
            },
            instruction.execute(cpu)
        );
    }
}
