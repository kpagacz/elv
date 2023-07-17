use crate::domain::errors::*;

pub trait InputCache {
    fn save(input: &str, year: u16, day: u8) -> Result<()>;
    fn load(year: u16, day: u8) -> Result<String>;
    fn clear() -> Result<()>;
}
