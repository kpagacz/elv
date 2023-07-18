use super::errors::InputCacheError;

pub trait InputCache {
    fn save(input: &str, year: u16, day: u8) -> Result<(), InputCacheError>;
    fn load(year: u16, day: u8) -> Result<String, InputCacheError>;
    fn clear() -> Result<(), InputCacheError>;
}
