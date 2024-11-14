pub trait FileLocation {
    fn get_line(&self) -> usize;
    fn get_column(&self) -> usize;
}
