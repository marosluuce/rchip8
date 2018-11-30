use cpu::Cpu;
use instructions::instruction::Instruction;
use instructions::op::Op;

struct Sprite {
    register1: usize,
    register2: usize,
    nibble: u8,
}

impl Op for Sprite {
    const MASK: u16 = 0xDFFF;
}

impl Instruction for Sprite {
    fn new(opcode: u16) -> Sprite {
        Sprite {
            register1: ((opcode & 0x0F00) >> 8) as usize,
            register2: ((opcode & 0x00F0) >> 4) as usize,
            nibble: (opcode & 0x000F) as u8,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draws_a_sprite() {}

    #[test]
    fn draw_a_sprite_wrapping_vertically() {}

    #[test]
    fn draw_a_sprite_wrapping_horizontally() {}

    #[test]
    fn draw_a_sprite_colliding() {}
}
