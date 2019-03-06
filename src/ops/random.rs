extern crate rand;
use self::rand::{thread_rng, Rng};

use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct Random {
    register: usize,
    value: u16,
    fixed_value: Option<u16>,
}

impl Random {
    fn get_value(&self) -> u8 {
        (self.fixed_value.unwrap_or(get_random(0, 255)) & self.value) as u8
    }
}

impl Op for Random {
    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu.set_register(self.register, self.get_value())
            .increment_pc()
    }
}

impl Matcher for Random {
    const MASK: u16 = 0xCFFF;

    fn new(opcode: u16) -> Random {
        Random {
            register: ((opcode & 0x0F00) >> 8) as usize,
            value: opcode & 0x00FF,
            fixed_value: None,
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
        let op = Random {
            fixed_value: Some(10),
            ..Random::new(0xCA22)
        };
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
        };

        assert_eq!(
            Cpu {
                pc: 6,
                ..cpu.set_register(0xA, 0x02)
            },
            op.execute(cpu)
        );
    }
}
