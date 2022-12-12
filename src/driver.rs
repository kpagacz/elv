use crate::errors::*;
use chrono::TimeZone;
use error_chain::bail;

use crate::aoc_api::{AocApi, ResponseStatus};
use crate::aoc_domain::{Submission, SubmissionStatus};
use crate::configuration::Configuration;
use crate::duration_string::DurationString;
use crate::input_cache::InputCache;
use crate::submission_history::SubmissionHistory;

const CACHE_SAVE_ERROR_MESSAGE: &str = concat!(
    "Could not save the submission result to the cache and the application will not ",
    "have any memory of this submission.",
    " Please check your permissions.",
    " If you are using Windows, please run the program as administrator.",
    " Be aware that the cache is used to prevent spamming the Advent of Code servers.",
    " If you spam the servers, your IP might be banned from the Advent of Code servers.",
);

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
            Err(e) => println!("Failed loading the input from the cache, cause: {}", e),
        };

        let aoc_api = AocApi::new(&self.configuration);
        let input = aoc_api.get_input(&year, &day);
        if input.status == ResponseStatus::Ok {
            if InputCache::cache(&input.body, year, day).is_err() {
                println!("Failed saving the input to the cache");
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
    ) {
        let aoc_api = AocApi::new(&self.configuration);

        let mut cache: Option<SubmissionHistory> = match SubmissionHistory::from_cache(year, day) {
            Ok(c) => Some(c),
            Err(e) => {
                println!("Error: {}", e);
                println!("The application will not have any memory of this submission.");
                None
            }
        };

        let submission = Submission::new(part, answer, year, day);
        if let Some(ref cache) = cache {
            if let Some(submission_result) = cache.get_result_for_submission(&submission) {
                println!("Your submission result:\n{:?}", submission_result.message);
                return;
            }
        }

        if let Some(ref cache) = cache {
            if let Some(wait_time) = cache.wait_time(chrono::Utc::now()) {
                println!("You wanted to submit an answer too soon. Please wait {} before submitting again.", DurationString::new(wait_time));
                return;
            }
        }

        let submission_result = aoc_api.submit_answer(submission);
        if submission_result.is_err() {
            println!("Error: {}", submission_result.err().unwrap());
            return;
        };
        let submission_result_unwrapped = submission_result.unwrap();
        println!(
            "Your submission result:\n{:?}",
            submission_result_unwrapped.message
        );
        if submission_result_unwrapped.status == SubmissionStatus::Correct
            || submission_result_unwrapped.status == SubmissionStatus::Incorrect
            || submission_result_unwrapped.status == SubmissionStatus::TooSoon
        {
            if let Some(ref mut cache) = cache {
                cache.add(submission_result_unwrapped);
                cache.save_to_cache().expect(CACHE_SAVE_ERROR_MESSAGE);
            } else {
                let mut cache = SubmissionHistory::new(year, day);
                cache.add(submission_result_unwrapped);
                cache.save_to_cache().expect(CACHE_SAVE_ERROR_MESSAGE);
            }
        }
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
        let input_release_time = match chrono::FixedOffset::west_opt(60 * 60 * 4)
            .unwrap()
            .with_ymd_and_hms(year as i32, 12, day as u32, 0, 0, 0)
            .single()
        {
            None => bail!("Invalid date"),
            Some(time) => time,
        };

        Ok(now >= &input_release_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_input_released_yet() {
        let driver = Driver::default();
        let now = chrono::Utc.with_ymd_and_hms(2022, 12, 1, 4, 0, 0).unwrap();
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
