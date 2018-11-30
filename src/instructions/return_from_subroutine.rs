use cpu::Cpu;
use instructions::instruction::Instruction;
use instructions::op::Op;

struct ReturnFromSubroutine;

impl Op for ReturnFromSubroutine {
    const MASK: u16 = 0x00EE;
}

impl Instruction for ReturnFromSubroutine {
    fn new(_opcode: u16) -> ReturnFromSubroutine {
        ReturnFromSubroutine {}
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        Cpu {
            pc: cpu.stack[cpu.sp as usize],
            sp: cpu.sp - 1,
            ..cpu
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executes_return_from_subroutine() {
        let instruction = ReturnFromSubroutine::new(0x00EE);
        let cpu = Cpu {
            stack: [2; 16],
            sp: 1,
            ..Cpu::new()
        };

        assert_eq!(
            Cpu {
                pc: 2,
                sp: 0,
                ..cpu
            },
            instruction.execute(cpu)
        );
    }
}
