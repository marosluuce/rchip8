use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

struct SetI {
    address: u16,
}

impl Instruction for SetI {
    const MASK: u16 = 0xAFFF;

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

impl fmt::Display for SetI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD I, {:X}", self.address)
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
