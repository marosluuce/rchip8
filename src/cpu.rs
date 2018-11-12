use instruction::{Instruction, Name};

pub struct Cpu {
    pub registers: Vec<u8>,
    pub i: u16,
    pub pc: u16,
    pub delay_timer: u16,
    pub sound_timer: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: vec![0; 16],
            i: 0,
            pc: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction {
                name: Name::SetI,
                args: Some(args),
                ..
            } => {
                self.i = args[0];
            }
            _ => panic!("Unknown instruction"),
        }
    }
}
