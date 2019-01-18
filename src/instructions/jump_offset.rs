use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

pub struct JumpOffset {
    address: u16,
}

impl Instruction for JumpOffset {
    const MASK: u16 = 0xBFFF;

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

impl fmt::Display for JumpOffset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JP V0, {:X}", self.address)
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
            ..Cpu::new()
        }
        .set_register(0, 4);

        assert_eq!(Cpu { pc: 0x126, ..cpu }, instruction.execute(cpu));
    }
}
