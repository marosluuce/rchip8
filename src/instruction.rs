use ops::op::Op;
use std::fmt;

pub struct Instruction {
    pub op: Box<dyn Op>,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.op)
    }
}
