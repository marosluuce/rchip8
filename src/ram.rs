#[derive(PartialEq, Debug)]
pub struct Ram {
    memory: Vec<u8>,
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            memory: vec![0; 0x1000],
        }
    }

    pub fn set(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn read(&mut self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}
