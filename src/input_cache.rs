use crate::errors::*;
use crate::Configuration;
use error_chain::bail;

pub struct InputCache {}

impl InputCache {
    fn cache_path(year: u16, day: u8) -> std::path::PathBuf {
        Configuration::get_project_directories()
            .cache_dir()
            .join("inputs")
            .join(format!("input-{}-{:02}", year, day))
    }

    pub fn cache(input: &str, year: u16, day: u8) -> Result<()> {
        let cache_path = Self::cache_path(year, day);
        if !cache_path.exists() {
            std::fs::create_dir_all(cache_path.parent().unwrap()).chain_err(|| {
                ErrorKind::CacheFailure(format!(
                    "Failed to create cache directory for {}-{:02}",
                    year, day
                ))
            })?;
        }
        std::fs::write(cache_path, input).chain_err(|| {
            ErrorKind::CacheFailure(format!(
                "Failed to write cached input for {}-{:02}",
                year, day
            ))
        })?;
        Ok(())
    }

    pub fn load(year: u16, day: u8) -> Result<String> {
        let cache_path = Self::cache_path(year, day);
        if !cache_path.exists() {
            bail!(ErrorKind::CacheFailure(format!(
                "No cached input for {}-{:02}",
                year, day
            )));
        }
        match std::fs::read_to_string(cache_path) {
            Ok(input) => Ok(input),
            Err(_) => bail!(ErrorKind::CacheFailure(format!(
                "Failed to read cached input for {}-{:02}",
                year, day
            ))),
        }
    }

    pub fn clear() -> Result<()> {
        let binding = Configuration::get_project_directories();
        let cache_dir = binding.cache_dir().join("inputs");
        if cache_dir.exists() {
            std::fs::remove_dir_all(cache_dir).chain_err(|| {
                ErrorKind::CacheFailure("Failed to remove the cache directory".to_string())
            })?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::InputCache;
    use crate::errors::*;

    #[test]
    fn can_cache_input() -> Result<()> {
        let input = "test input";
        let year = 1000;
        let day = 1;
        InputCache::cache(input, year, day)?;
        let cached_input = InputCache::load(year, day)?;
        assert_eq!(input, cached_input);
        Ok(())
    }

    #[test]
    fn can_clear_cache() -> Result<()> {
        let input = "test input";
        let year = 1000;
        let day = 2;
        InputCache::cache(input, year, day)?;
        let cached_input = InputCache::load(year, day)?;
        assert_eq!(input, cached_input);
        InputCache::clear()?;
        assert!(InputCache::load(year, day).is_err());
        Ok(())
    }

    #[test]
    fn can_clear_cache_with_no_cache() -> Result<()> {
        let year = 1000;
        let day = 3;
        InputCache::clear()?;
        assert!(InputCache::load(year, day).is_err());
        Ok(())
    }
}
