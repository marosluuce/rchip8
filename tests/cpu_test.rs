extern crate rchip8;

#[cfg(test)]
mod tests {
    use rchip8::cpu::Cpu;

    #[test]
    fn executes_set_i() {
        let cpu = Cpu::new();

        let updated = cpu.execute(0xAAAB);

        assert_eq!(Cpu {i: 0xAAB, ..cpu}, updated);
    }

    #[test]
    fn executes_jump_absolute() {
        let cpu = Cpu::new();

        let updated = cpu.execute(0x1123);

        assert_eq!(Cpu {pc: 0x123, ..cpu}, updated);
    }

    #[test]
    fn skips_when_register_equals_value() {
        //TODO: Find a better way to set registers.
        let cpu = Cpu {pc: 15, registers: [0x45; 16], ..Cpu::new()};

        let updated = cpu.execute(0x3145);

        assert_eq!(Cpu {pc: 17, ..cpu}, updated);
    }
}
