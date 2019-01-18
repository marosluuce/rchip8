use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

pub struct Subtract {
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
        cpu.update_register(0xF, |_| {
            (cpu.registers[self.register1] > cpu.registers[self.register2]) as u8
        })
        .update_register(self.register1, |x| {
            x.wrapping_sub(cpu.registers[self.register2])
        })
        .increment_pc()
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
            ..Cpu::new()
        }
        .set_register(2, 2)
        .set_register(4, 2)
        .set_register(0xF, 5);

        assert_eq!(
            Cpu {
                pc: 6,
                ..cpu
                    .set_register(2, 0)
                    .set_register(4, 2)
                    .set_register(0xF, 0)
            },
            instruction.execute(cpu)
        );
    }

    #[test]
    fn subtracts_two_registers_with_no_borrow() {
        let instruction = Subtract::new(0x8245);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
        }
        .set_register(2, 5)
        .set_register(4, 3);

        assert_eq!(
            Cpu {
                pc: 6,
                ..cpu
                    .set_register(2, 2)
                    .set_register(4, 3)
                    .set_register(0xF, 1)
            },
            instruction.execute(cpu)
        );
    }
}
