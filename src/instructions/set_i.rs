use cpu::Cpu;
use instructions::instruction::Instruction;
use instructions::op::Op;

struct SetI {
    address: u16,
}

impl Op for SetI {
    const MASK: u16 = 0xAFFF;
}

impl Instruction for SetI {
    fn new(opcode: u16) -> SetI {
        SetI {
            address: opcode & 0x0FFF,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        Cpu {
            pc: cpu.pc + 1,
            i: self.address,
            ..cpu
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_i_address() {
        let instruction = SetI::new(0xA123);
        let cpu = Cpu {
            pc: 4,
            ..Cpu::new()
        };

        assert_eq!(
            Cpu {
                pc: 5,
                i: 0x123,
                ..cpu
            },
            instruction.execute(cpu)
        );
    }
}
