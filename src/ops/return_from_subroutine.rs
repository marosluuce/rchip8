use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct ReturnFromSubroutine;

impl Op for ReturnFromSubroutine {
    fn execute(&self, cpu: Cpu) -> Cpu {
        Cpu {
            pc: cpu.stack[cpu.sp as usize],
            sp: cpu.sp - 1,
            ..cpu
        }
    }
}

impl Matcher for ReturnFromSubroutine {
    const MASK: u16 = 0x00EE;

    fn new(_opcode: u16) -> ReturnFromSubroutine {
        ReturnFromSubroutine {}
    }
}

impl fmt::Display for ReturnFromSubroutine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RET")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executes_return_from_subroutine() {
        let op = ReturnFromSubroutine::new(0x00EE);
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
            op.execute(cpu)
        );
    }
}
