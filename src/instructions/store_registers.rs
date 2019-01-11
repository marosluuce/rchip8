use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

struct StoreRegisters {
    register: usize,
}

impl Instruction for StoreRegisters {
    const MASK: u16 = 0xFF55;

    fn new(opcode: u16) -> StoreRegisters {
        StoreRegisters {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}

impl fmt::Display for StoreRegisters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD [I], V{:X}", self.register)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
