extern crate rchip8;

#[cfg(test)]
mod tests {
    use rchip8::ram::Ram;

    #[test]
    fn creates_ram() {
        assert_eq!(Ram::new(), Ram::new())
    }

    #[test]
    fn sets_an_address() {
        let mut ram = Ram::new();
        ram.set(0x999, 0b11001110);

        assert_eq!(0b11001110, ram.read(0x999))
    }
}
