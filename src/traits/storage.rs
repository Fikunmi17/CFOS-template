pub trait Storage {
    fn read(&mut self, buffer: &mut [u8]);
    fn write(&mut self, buffer: &[u8]);
}
