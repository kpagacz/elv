use crate::domain::ports::{InputCache, InputCacheError};
use crate::Configuration;

pub struct FileInputCache;

impl FileInputCache {
    fn cache_path(year: u16, day: u8) -> std::path::PathBuf {
        Configuration::get_project_directories()
            .cache_dir()
            .join("inputs")
            .join(format!("input-{}-{:02}", year, day))
    }
}

impl From<std::io::Error> for InputCacheError {
    fn from(_cause: std::io::Error) -> Self {
        Self::Save
    }
}

impl InputCache for FileInputCache {
    fn save(input: &str, year: u16, day: u8) -> Result<(), InputCacheError> {
        let cache_path = Self::cache_path(year, day);
        if !cache_path.exists() {
            std::fs::create_dir_all(cache_path.parent().unwrap())?;
        }
        std::fs::write(cache_path, input)?;
        Ok(())
    }

    fn load(year: u16, day: u8) -> Result<String, InputCacheError> {
        let cache_path = Self::cache_path(year, day);
        if !cache_path.exists() {
            return Err(InputCacheError::Empty(format!(
                "No cached input for {}-{:02}",
                year, day
            )));
        }
        match std::fs::read_to_string(cache_path) {
            Ok(input) => Ok(input),
            Err(_) => Err(InputCacheError::Load(format!(
                "Failed to read cached input for {}-{:02}",
                year, day
            ))),
        }
    }

    fn clear() -> Result<(), InputCacheError> {
        let binding = Configuration::get_project_directories();
        let cache_dir = binding.cache_dir().join("inputs");
        if cache_dir.exists() {
            std::fs::remove_dir_all(cache_dir).map_err(|_| {
                InputCacheError::Clear("Failed to remove the cache directory".to_string())
            })?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::FileInputCache;
    use crate::domain::ports::{InputCache, InputCacheError};

    #[test]
    fn cache_tests() -> Result<(), InputCacheError> {
        let input = "test input";
        let year = 1000;
        let day = 1;
        FileInputCache::save(input, year, day)?;
        let cached_input = FileInputCache::load(year, day)?;
        assert_eq!(input, cached_input);

        FileInputCache::clear()?;
        assert!(FileInputCache::load(year, day).is_err());

        Ok(())
    }
}
