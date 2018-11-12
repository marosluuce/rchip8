extern crate rchip8;

#[cfg(test)]
mod tests {
    use rchip8::cpu::Cpu;
    use rchip8::instruction::{Instruction, Name};

    #[test]
    fn executes_set_i() {
        let mut cpu = Cpu::new();

        cpu.execute(Instruction {
            name: Name::SetI,
            args: Some(vec![0xAAB]),
        });

        assert_eq!(cpu.i, 0xAAB);
    }
}
