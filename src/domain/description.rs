pub trait Description: std::fmt::Display {
    fn year(&self) -> u16;
    fn day(&self) -> u8;
}
