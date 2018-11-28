use cpu::Cpu;
use instructions::instruction::Instruction;
use instructions::op::Op;

struct SkipEqualsAbsolute {
    register: usize,
    value: u8,
}

impl Op for SkipEqualsAbsolute {
    const MASK: u16 = 0x3FFF;
}

impl Instruction for SkipEqualsAbsolute {
    fn new(opcode: u16) -> SkipEqualsAbsolute {
        SkipEqualsAbsolute {
            register: (opcode & 0x0F00 >> 4) as usize,
            value: (opcode & 0x00FF) as u8,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        if cpu.registers[self.register] == self.value {
            Cpu { pc: cpu.pc + 2, ..cpu }
        } else {
            Cpu { pc: cpu.pc + 1, ..cpu }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executes_skip_when_register_equals_value() {
        let cpu = Cpu {
            pc: 4,
            registers: [4; 16],
            ..Cpu::new()
        };
        let instruction = SkipEqualsAbsolute::new(0x3204);

        let updated = instruction.execute(cpu);

        assert_eq!(
            Cpu {
                pc: 6,
                sp: 0,
                ..cpu
            },
            updated
        );
    }

    #[test]
    fn executes_does_not_skip() {
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
        };
        let instruction = SkipEqualsAbsolute::new(0x3205);

        let updated = instruction.execute(cpu);

        assert_eq!(Cpu { pc: 5, ..cpu }, updated);
    }
}
