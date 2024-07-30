pub trait Keyboard {
    fn read_key(&mut self) -> Option<u8>;
}
