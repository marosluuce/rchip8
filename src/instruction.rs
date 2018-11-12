lazy_static! {
    static ref OPERATIONS: Vec<Op> = vec![
        Op {
            name: Name::ClearScreen,
            code: 0x00E0,
            masks: None,
        },
        Op {
            name: Name::ReturnFromSubroutine,
            code: 0x00EE,
            masks: None,
        },
        Op {
            name: Name::JumpAbsolute,
            code: 0x1FFF,
            masks: Some(vec![0x0FFF]),
        },
        Op {
            name: Name::JumpStack,
            code: 0x2FFF,
            masks: Some(vec![0x0FFF]),
        },
        Op {
            name: Name::SkipEqualAbsolute,
            code: 0x3FFF,
            masks: Some(vec![0x0F00, 0x00FF]),
        },
        Op {
            name: Name::SkipNotEqualAbsolute,
            code: 0x4FFF,
            masks: Some(vec![0x0F00, 0x00FF]),
        },
        Op {
            name: Name::SkipEqual,
            code: 0x5FFF,
            masks: Some(vec![0x0F00, 0x00F0]),
        },
        Op {
            name: Name::Move,
            code: 0x8FF0,
            masks: Some(vec![0x0F00, 0x00F0]),
        },
        Op {
            name: Name::Or,
            code: 0x8FF1,
            masks: Some(vec![0x0F00, 0x00F0]),
        },
        Op {
            name: Name::And,
            code: 0x8FF2,
            masks: Some(vec![0x0F00, 0x00F0]),
        },
        Op {
            name: Name::Xor,
            code: 0x8FF3,
            masks: Some(vec![0x0F00, 0x00F0]),
        },
        Op {
            name: Name::Add,
            code: 0x8FF4,
            masks: Some(vec![0x0F00, 0x00F0]),
        },
        Op {
            name: Name::Subtract,
            code: 0x8FF5,
            masks: Some(vec![0x0F00, 0x00F0]),
        },
        Op {
            name: Name::ShiftRight,
            code: 0x8FF6,
            masks: Some(vec![0x0F00]),
        },
        Op {
            name: Name::SubtractBackwards,
            code: 0x8FF7,
            masks: Some(vec![0x0F00, 0x00F0]),
        },
        Op {
            name: Name::ShiftLeft,
            code: 0x8FFE,
            masks: Some(vec![0x0F00]),
        },
        Op {
            name: Name::SkipNotEqual,
            code: 0x9FFF,
            masks: Some(vec![0x0F00, 0x00F0]),
        },
        Op {
            name: Name::SetI,
            code: 0xAFFF,
            masks: Some(vec![0x0FFF]),
        },
        Op {
            name: Name::Sprite,
            code: 0xDFFF,
            masks: Some(vec![0x0F00, 0x00F0, 0x000F])
        },
        Op {
            name: Name::SkipKey,
            code: 0xEF9E,
            masks: Some(vec![0x0F00])
        },
        Op {
            name: Name::SkipNoKey,
            code: 0xEFA1,
            masks: Some(vec![0x0F00])
        },
        Op {
            name: Name::MoveDelay,
            code: 0xFF07,
            masks: Some(vec![0x0F00])
        },
        Op {
            name: Name::WaitKey,
            code: 0xFF0A,
            masks: Some(vec![0x0F00])
        }
    ];
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Name {
    ClearScreen,
    ReturnFromSubroutine,
    JumpAbsolute,
    JumpStack,
    SkipEqualAbsolute,
    SkipNotEqualAbsolute,
    SkipEqual,
    SkipNotEqual,
    Move,
    Or,
    And,
    Xor,
    Add,
    Subtract,
    ShiftRight,
    SubtractBackwards,
    ShiftLeft,
    SetI,
    Sprite,
    SkipKey,
    SkipNoKey,
    MoveDelay,
    WaitKey,
    MovDelayReverse,
    MoveSound,
    AddAbsolute,
    SpriteChar,
    MoveBCD,
    MoveM,
    MoveMReverse,
}

#[derive(Debug, PartialEq)]
struct Op {
    name: Name,
    code: u16,
    masks: Option<Vec<u16>>,
}

impl Op {
    pub fn matches(&self, opcode: u16) -> bool {
        opcode & self.code == opcode
    }

    pub fn extract_args(&self, opcode: u16) -> Option<Vec<u16>> {
        self.masks.clone().map(|masks| {
            masks
                .into_iter()
                .map(|mask| opcode & mask)
                .collect::<Vec<u16>>()
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct Instruction {
    pub name: Name,
    pub args: Option<Vec<u16>>,
}

impl Instruction {
    pub fn new(opcode: u16) -> Instruction {
        for op in OPERATIONS.iter() {
            if op.matches(opcode) {
                return Instruction {
                    name: op.name,
                    args: op.extract_args(opcode),
                };
            }
        }

        panic!("Unknown opcode")
    }
}
