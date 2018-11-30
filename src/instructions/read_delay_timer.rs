use cpu::Cpu;
use instructions::instruction::Instruction;
use instructions::op::Op;

struct ReadDelayTimer {
    register: usize,
}

impl Op for ReadDelayTimer {
    const MASK: u16 = 0xFF07;
}

impl Instruction for ReadDelayTimer {
    fn new(opcode: u16) -> ReadDelayTimer {
        ReadDelayTimer {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        Cpu {
            pc: cpu.pc + 1,
            registers: {
                let mut registers = cpu.registers;
                registers[self.register] = cpu.delay_timer;
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
    fn loads_the_delay_timer_into_register() {
        let instruction = ReadDelayTimer::new(0xFD07);
        let cpu = Cpu {
            pc: 4,
            delay_timer: 0x1111,
            ..Cpu::new()
        };

        assert_eq!(
            Cpu {
                pc: 5,
                registers: {
                    let mut registers = [0; 16];
                    registers[0xD] = 0x11;
                    registers
                },
                ..cpu
            },
            instruction.execute(cpu)
        );
    }
}
