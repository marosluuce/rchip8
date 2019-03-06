use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct ReadDelayTimer {
    register: usize,
}

impl Op for ReadDelayTimer {
    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu.set_register(self.register, cpu.delay_timer)
            .increment_pc()
    }
}

impl Matcher for ReadDelayTimer {
    const MASK: u16 = 0xFF07;

    fn new(opcode: u16) -> ReadDelayTimer {
        ReadDelayTimer {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }
}

impl fmt::Display for ReadDelayTimer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD DT, V{:X}", self.register)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_the_delay_timer_into_register() {
        let op = ReadDelayTimer::new(0xFD07);
        let cpu = Cpu {
            pc: 4,
            delay_timer: 0x11,
            ..Cpu::new()
        };

        assert_eq!(
            Cpu {
                pc: 6,
                ..cpu.set_register(0xD, 0x11)
            },
            op.execute(cpu)
        );
    }
}
