extern crate rand;
use self::rand::{thread_rng, Rng};

use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

struct Random {
    register: usize,
    value: u16,
    fixed_value: Option<u16>,
}

impl Random {
    fn get_value(&self) -> u8 {
        (self.fixed_value.unwrap_or(get_random(0, 255)) & self.value) as u8
    }
}

impl Instruction for Random {
    const MASK: u16 = 0xCFFF;

    fn new(opcode: u16) -> Random {
        Random {
            register: ((opcode & 0x0F00) >> 8) as usize,
            value: opcode & 0x00FF,
            fixed_value: None,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        Cpu {
            pc: cpu.pc + 1,
            registers: {
                let mut registers = cpu.registers;
                registers[self.register] = self.get_value();
                registers
            },
            ..cpu
        }
    }
}

impl fmt::Display for Random {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RND V{:X}, {:X}", self.register, self.get_value())
    }
}

fn get_random(min: u16, max: u16) -> u16 {
    let mut rng = thread_rng();
    rng.gen_range(min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_a_random_number() {
        let instruction = Random {
            fixed_value: Some(10),
            ..Random::new(0xCA22)
        };
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
        };

        assert_eq!(
            Cpu {
                pc: 5,
                registers: {
                    let mut registers = [0; 16];
                    registers[0xA] = 0x02;
                    registers
                },
                ..cpu
            },
            instruction.execute(cpu)
        );
    }
}
