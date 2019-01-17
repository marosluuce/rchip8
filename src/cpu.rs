#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cpu {
    pub registers: [u8; 16],
    pub stack: [u16; 16],
    pub i: u16,
    pub pc: u16,
    pub sp: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: [0; 16],
            stack: [0; 16],
            i: 0,
            pc: 0,
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}
