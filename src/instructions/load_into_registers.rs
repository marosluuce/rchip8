use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

struct LoadIntoRegisters {
    register: usize,
}

impl Instruction for LoadIntoRegisters {
    const MASK: u16 = 0xFF65;

    fn new(opcode: u16) -> LoadIntoRegisters {
        LoadIntoRegisters {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        Cpu {
            pc: cpu.pc + 1,
            ..cpu
        }
    }
}

impl fmt::Display for LoadIntoRegisters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD V{:X}, [I]", self.register)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
