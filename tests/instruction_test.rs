extern crate rchip8;

#[cfg(test)]
mod tests {
    use rchip8::instruction::{Instruction, Name};

    #[test]
    fn decodes_clear_screen() {
        assert_eq!(
            Instruction::new(0x00E0),
            Instruction {
                name: Name::ClearScreen,
                args: None
            }
        );
    }

    #[test]
    fn decodes_return_from_subroutine() {
        assert_eq!(
            Instruction::new(0x00EE),
            Instruction {
                name: Name::ReturnFromSubroutine,
                args: None
            }
        );
    }

    #[test]
    fn decodes_jump_absolute() {
        assert_eq!(
            Instruction::new(0x1234),
            Instruction {
                name: Name::JumpAbsolute,
                args: Some(vec![0x234])
            }
        );
    }

    #[test]
    fn decodes_jump_stack() {
        assert_eq!(
            Instruction::new(0x2234),
            Instruction {
                name: Name::JumpStack,
                args: Some(vec![0x234])
            }
        );
    }

    #[test]
    fn decodes_skip_equal_absolute() {
        assert_eq!(
            Instruction::new(0x3A00),
            Instruction {
                name: Name::SkipEqualAbsolute,
                args: Some(vec![0xA00, 0x00])
            }
        );
    }

    #[test]
    fn decodes_skip_not_equal_absolute() {
        assert_eq!(
            Instruction::new(0x4800),
            Instruction {
                name: Name::SkipNotEqualAbsolute,
                args: Some(vec![0x800, 0x00])
            }
        );
    }

    #[test]
    fn decodes_skip_equal() {
        assert_eq!(
            Instruction::new(0x5A70),
            Instruction {
                name: Name::SkipEqual,
                args: Some(vec![0xA00, 0x70])
            }
        );
    }

    #[test]
    fn decodes_load_absolute() {
        assert_eq!(
            Instruction::new(0x6A23),
            Instruction {
                name: Name::LoadAbsolute,
                args: Some(vec![0xA00, 0x23])
            }
        );
    }

    #[test]
    fn decodes_add_absolute() {
        assert_eq!(
            Instruction::new(0x7145),
            Instruction {
                name: Name::AddAbsolute,
                args: Some(vec![0x100, 0x45])
            }
        )
    }

    #[test]
    fn decodes_move() {
        assert_eq!(
            Instruction::new(0x83A0),
            Instruction {
                name: Name::Move,
                args: Some(vec![0x300, 0xA0])
            }
        );
    }

    #[test]
    fn decodes_or() {
        assert_eq!(
            Instruction::new(0x83A1),
            Instruction {
                name: Name::Or,
                args: Some(vec![0x300, 0xA0])
            }
        );
    }

    #[test]
    fn decodes_and() {
        assert_eq!(
            Instruction::new(0x83A2),
            Instruction {
                name: Name::And,
                args: Some(vec![0x300, 0xA0])
            }
        );
    }

    #[test]
    fn decodes_xor() {
        assert_eq!(
            Instruction::new(0x83A3),
            Instruction {
                name: Name::Xor,
                args: Some(vec![0x300, 0xA0])
            }
        );
    }

    #[test]
    fn decodes_add() {
        assert_eq!(
            Instruction::new(0x83A4),
            Instruction {
                name: Name::Add,
                args: Some(vec![0x300, 0xA0])
            }
        );
    }

    #[test]
    fn decodes_subtract() {
        assert_eq!(
            Instruction::new(0x83A5),
            Instruction {
                name: Name::Subtract,
                args: Some(vec![0x300, 0xA0])
            }
        );
    }

    #[test]
    fn decodes_shift_right() {
        assert_eq!(
            Instruction::new(0x83A6),
            Instruction {
                name: Name::ShiftRight,
                args: Some(vec![0x300])
            }
        );
    }

    #[test]
    fn decodes_subtract_backwards() {
        assert_eq!(
            Instruction::new(0x83A7),
            Instruction {
                name: Name::SubtractBackwards,
                args: Some(vec![0x300, 0xA0])
            }
        );
    }

    #[test]
    fn decodes_shift_left() {
        assert_eq!(
            Instruction::new(0x83AE),
            Instruction {
                name: Name::ShiftLeft,
                args: Some(vec![0x300])
            }
        );
    }

    #[test]
    fn decodes_skip_not_equal() {
        assert_eq!(
            Instruction::new(0x93B0),
            Instruction {
                name: Name::SkipNotEqual,
                args: Some(vec![0x300, 0xB0])
            }
        );
    }

    #[test]
    fn decodes_set_i() {
        assert_eq!(
            Instruction::new(0xAAAB),
            Instruction {
                name: Name::SetI,
                args: Some(vec![0xAAB])
            }
        );
    }

    #[test]
    fn decodes_jump_offset() {
        assert_eq!(
            Instruction::new(0xBAAB),
            Instruction {
                name: Name::JumpOffset,
                args: Some(vec![0xAAB])
            }
        );
    }

    #[test]
    fn decodes_rand() {
        assert_eq!(
            Instruction::new(0xCB97),
            Instruction {
                name: Name::Random,
                args: Some(vec![0xB00, 0x97])
            }
        );
    }

    #[test]
    fn decodes_sprite() {
        assert_eq!(
            Instruction::new(0xD123),
            Instruction {
                name: Name::Sprite,
                args: Some(vec![0x100, 0x20, 0x3])
            }
        );
    }

    #[test]
    fn decodes_skip_key() {
        assert_eq!(
            Instruction::new(0xE29E),
            Instruction {
                name: Name::SkipKey,
                args: Some(vec![0x200])
            }
        )
    }

    #[test]
    fn decodes_skip_no_key() {
        assert_eq!(
            Instruction::new(0xE2A1),
            Instruction {
                name: Name::SkipNoKey,
                args: Some(vec![0x200])
            }
        )
    }

    #[test]
    fn decodes_move_delay() {
        assert_eq!(
            Instruction::new(0xF307),
            Instruction {
                name: Name::MoveDelay,
                args: Some(vec![0x300])
            }
        )
    }

    #[test]
    fn decodes_wait_delay() {
        assert_eq!(
            Instruction::new(0xF10A),
            Instruction {
                name: Name::WaitKey,
                args: Some(vec![0x100])
            }
        )
    }

    #[test]
    fn decodes_set_delay_timer() {
        assert_eq!(
            Instruction::new(0xF115),
            Instruction {
                name: Name::SetDelayTimer,
                args: Some(vec![0x100])
            }
        )
    }

    #[test]
    fn decodes_set_sound_timer() {
        assert_eq!(
            Instruction::new(0xF218),
            Instruction {
                name: Name::SetSoundTimer,
                args: Some(vec![0x200])
            }
        )
    }

    #[test]
    fn decodes_add_to_instruction_point() {
        assert_eq!(
            Instruction::new(0xF31E),
            Instruction {
                name: Name::AddToInstructionPointer,
                args: Some(vec![0x300])
            }
        )
    }

    #[test]
    fn decodes_load_sprite_location() {
        assert_eq!(
            Instruction::new(0xF429),
            Instruction {
                name: Name::LoadSpriteLocation,
                args: Some(vec![0x400])
            }
        )
    }

    #[test]
    fn decodes_store_bcd() {
        assert_eq!(
            Instruction::new(0xF233),
            Instruction {
                name: Name::StoreBCD,
                args: Some(vec![0x200])
            }
        )
    }

    #[test]
    fn decodes_store_registers() {
        assert_eq!(
            Instruction::new(0xF555),
            Instruction {
                name: Name::StoreRegisters,
                args: Some(vec![0x500])
            }
        )
    }

    #[test]
    fn decodes_load_into_registers() {
        assert_eq!(
            Instruction::new(0xF165),
            Instruction {
                name: Name::LoadIntoRegisters,
                args: Some(vec![0x100])
            }
        )
    }
}
