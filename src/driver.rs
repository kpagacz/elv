use std::collections::HashMap;

use crate::errors::*;
use chrono::TimeZone;
use error_chain::bail;

use crate::aoc_api::{AocApi, ResponseStatus};
use crate::aoc_domain::{Submission, SubmissionStatus};
use crate::configuration::Configuration;
use crate::duration_string::DurationString;
use crate::input_cache::InputCache;
use crate::submission_history::SubmissionHistory;

#[derive(Debug, Default)]
pub struct Driver {
    configuration: Configuration,
}

impl Driver {
    pub fn new(configuration: Configuration) -> Self {
        Self { configuration }
    }

    pub fn input(&self, year: u16, day: u8) -> Result<String> {
        let is_already_released = match self.is_input_released_yet(year, day, &chrono::Utc::now()) {
            Ok(released) => released,
            Err(e) => bail!(Error::with_chain(
                e,
                "Failed to check if the input is released yet"
            )),
        };
        if !is_already_released {
            bail!("The input is not released yet");
        }

        match InputCache::load(year, day) {
            Ok(input) => return Ok(input),
            Err(e) => eprintln!("Failed loading the input from the cache. Cause:\n    {}", e),
        };

        let aoc_api = AocApi::new(&self.configuration);
        let input = aoc_api.get_input(&year, &day);
        if input.status == ResponseStatus::Ok {
            if InputCache::cache(&input.body, year, day).is_err() {
                eprintln!("Failed saving the input to the cache");
            }
        } else {
            bail!("Failed to get the input: {}", input.body);
        }
        Ok(input.body)
    }

    pub fn submit_answer(
        &self,
        year: u16,
        day: u8,
        part: crate::aoc_domain::RiddlePart,
        answer: String,
    ) -> Result<()> {
        let aoc_api = AocApi::new(&self.configuration);

        let mut cache: Option<SubmissionHistory> = match SubmissionHistory::from_cache(&year, &day)
        {
            Ok(c) => Some(c),
            Err(e) => {
                eprintln!("Error: {}", e);
                eprintln!("The application will not have any memory of this submission.");
                None
            }
        };

        let submission = Submission::new(part, answer, year, day);
        if let Some(ref cache) = cache {
            if let Some(submission_result) = cache.correct_submission(&submission.part) {
                eprintln!("🎉  You already submitted the correct answer for this part. Here is the result from last time...\n\n");
                println!("{}", submission_result.message);
                return Ok(());
            }

            if let Some(submission_result) = cache.get_result_for_submission(&submission) {
                eprintln!("♻️  You submitted this answer before and the result was...\n\n");
                println!("{}", submission_result.message);
                if let Some(wait_time) = cache.wait_time(&chrono::Utc::now(), &submission.part) {
                    eprintln!(
                        "\n🌡️  You still need to wait {} before another submission.",
                        DurationString::new(wait_time)
                    );
                }
                return Ok(());
            }

            if let Some(wait_time) = cache.wait_time(&chrono::Utc::now(), &submission.part) {
                eprintln!("🌡️  You wanted to submit an answer too soon. Please wait {} before submitting again.",
                DurationString::new(wait_time));
                return Ok(());
            }
        }

        let submission_result = aoc_api
            .submit_answer(submission)
            .chain_err(|| "Failed to submit the answer")?;
        eprintln!("Your submission result...\n\n");
        println!("{}", submission_result.message);
        if submission_result.status == SubmissionStatus::Correct
            || submission_result.status == SubmissionStatus::Incorrect
            || submission_result.status == SubmissionStatus::TooSoon
        {
            if let Some(ref mut cache) = cache {
                cache.add(submission_result);
                return cache.save_to_cache();
            } else {
                let mut cache = SubmissionHistory::new(year, day);
                cache.add(submission_result);
                return cache.save_to_cache();
            }
        }

        Ok(())
    }

    pub fn clear_cache(&self) -> Result<()> {
        InputCache::clear().chain_err(|| "Failed to clear the input cache")?;
        SubmissionHistory::clear().chain_err(|| "Failed to clear the submission history cache")?;
        Ok(())
    }

    /// Returns the description of the riddles
    pub fn get_description(&self, year: u16, day: u8) -> Result<String> {
        let aoc_api = AocApi::new(&self.configuration);
        Ok(aoc_api.get_description(&year, &day)?)
    }

    fn is_input_released_yet(
        &self,
        year: u16,
        day: u8,
        now: &chrono::DateTime<chrono::Utc>,
    ) -> Result<bool> {
        let input_release_time = match chrono::FixedOffset::west_opt(60 * 60 * 5)
            .unwrap()
            .with_ymd_and_hms(year as i32, 12, day as u32, 0, 0, 0)
            .single()
        {
            None => bail!("Invalid date"),
            Some(time) => time,
        };

        Ok(now >= &input_release_time)
    }
    /// Lists the directories used by the application
    /// # Example
    /// ```
    /// use elv::Driver;
    /// let driver = Driver::default();
    /// driver.list_app_directories();
    /// ```
    pub fn list_app_directories(&self) -> Result<HashMap<&str, String>> {
        let mut directories = HashMap::new();
        if let Some(config_dir) = Configuration::get_project_directories()
            .config_dir()
            .to_str()
        {
            directories.insert("config", config_dir.to_owned());
        }
        if let Some(cache_dir) = Configuration::get_project_directories()
            .cache_dir()
            .to_str()
        {
            directories.insert("cache", cache_dir.to_owned());
        }
        Ok(directories)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_input_released_yet() {
        let driver = Driver::default();
        let now = chrono::Utc.with_ymd_and_hms(2022, 12, 1, 5, 0, 0).unwrap();
        for (year, day, expected) in &[
            (2019, 1, true),
            (2020, 1, true),
            (2021, 1, true),
            (2022, 1, true),
            (2022, 2, false),
            (2023, 1, false),
            (2024, 1, false),
        ] {
            assert_eq!(
                driver.is_input_released_yet(*year, *day, &now).unwrap(),
                *expected,
                "Input for {}-{} should be released: {}",
                year,
                day,
                expected
            );
        }
    }

    #[test]
    fn test_invalid_date_to_input() {
        let driver = Driver::default();
        let input = driver.input(0, 0);
        assert!(input.is_err());
        let error = input.err().unwrap();
        assert!(
            error.description() == "Failed to check if the input is released yet",
            "Error message should be 'Failed to check if the input is released yet', was: {}",
            error.description()
        );
    }
}
