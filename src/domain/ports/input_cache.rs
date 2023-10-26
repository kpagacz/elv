use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputCacheError {
    #[error("Failed saving to cache")]
    Save,

    #[error("Failed to load from cache")]
    Load(String),

    #[error("Failed to clear the cache: {}", 0)]
    Clear(String),

    #[error("{}", 0)]
    Empty(String),
}

pub trait InputCache {
    fn save(input: &str, year: usize, day: usize) -> Result<(), InputCacheError>;
    fn load(year: usize, day: usize) -> Result<String, InputCacheError>;
    fn clear() -> Result<(), InputCacheError>;
}
