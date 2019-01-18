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

    pub fn set_register(&self, register: usize, value: u8) -> Cpu {
        Cpu {
            registers: {
                let mut registers = self.registers;
                registers[register] = value;
                registers
            },
            ..*self
        }
    }

    pub fn update_register<F>(&self, register: usize, f: F) -> Cpu
    where
        F: Fn(u8) -> u8,
    {
        self.set_register(register, f(self.registers[register]))
    }

    pub fn increment_pc(&self) -> Cpu {
        Cpu {
            pc: self.pc + 2,
            ..*self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_register() {
        assert_eq!(
            Cpu {
                registers: {
                    let mut registers = [0; 16];
                    registers[2] = 16;
                    registers
                },
                ..Cpu::new()
            },
            Cpu::new().set_register(2, 16)
        );
    }

    #[test]
    fn update_register() {
        assert_eq!(
            Cpu {
                registers: {
                    let mut registers = [0; 16];
                    registers[2] = 16;
                    registers
                },
                ..Cpu::new()
            },
            Cpu::new().update_register(2, |x| x + 16)
        );
    }

    #[test]
    fn increment_pc() {
        assert_eq!(
            Cpu {
                pc: 4,
                ..Cpu::new()
            },
            Cpu {
                pc: 2,
                ..Cpu::new()
            }
            .increment_pc()
        );
    }
}
