use cpu::Cpu;
use instructions::instruction::Instruction;
use instructions::op::Op;

struct JumpOffset {
    address: u16,
}

impl Op for JumpOffset {
    const MASK: u16 = 0xBFFF;
}

impl Instruction for JumpOffset {
    fn new(opcode: u16) -> JumpOffset {
        JumpOffset {
            address: opcode & 0x0FFF,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        Cpu {
            pc: cpu.registers[0] as u16 + self.address,
            ..cpu
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jumps_to_an_address_offset() {
        let instruction = JumpOffset::new(0xB122);
        let cpu = Cpu {
            pc: 4,
            registers: {
                let mut registers = [0; 16];
                registers[0] = 0x04;
                registers
            },
            ..Cpu::new()
        };

        assert_eq!(Cpu { pc: 0x126, ..cpu }, instruction.execute(cpu));
    }
}
