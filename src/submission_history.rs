use crate::errors::*;
use crate::{
    aoc_domain::{Submission, SubmissionResult},
    configuration::Configuration,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionHistory {
    submissions: Vec<SubmissionResult>,
    year: u16,
    day: u8,
}

impl SubmissionHistory {
    pub fn new(year: u16, day: u8) -> Self {
        SubmissionHistory {
            submissions: Vec::new(),
            year,
            day,
        }
    }

    pub fn from_cache(year: u16, day: u8) -> Result<Self> {
        let cache_path = Self::cache_path(year, day);
        if !cache_path.exists() {
            Self::new(year, day).save_to_cache()?;
        }
        let content = std::fs::read(&cache_path).chain_err(|| {
            ErrorKind::CacheFailure(format!(
                "Failed to read cache file: {}",
                cache_path.display()
            ))
        })?;
        serde_cbor::from_slice::<SubmissionHistory>(&content).chain_err(|| {
            ErrorKind::CacheFailure(format!(
                "Failed to deserialize cache file: {}",
                cache_path.display()
            ))
        })
    }

    pub fn add(&mut self, submission: SubmissionResult) {
        self.submissions.push(submission);
    }

    pub fn can_submit(&self, now: chrono::DateTime<chrono::Utc>) -> bool {
        self.submissions.is_empty()
            || self.submissions.last().unwrap().submitted_at
                + chrono::Duration::minutes(self.submissions.last().unwrap().wait_minutes)
                < now
    }

    pub fn wait_time(&self, now: chrono::DateTime<chrono::Utc>) -> Option<chrono::Duration> {
        if self.can_submit(now) {
            None
        } else {
            let remaining = self.submissions.last().unwrap().submitted_at
                + chrono::Duration::minutes(self.submissions.last().unwrap().wait_minutes)
                - now;
            Some(remaining)
        }
    }

    pub fn get_result_for_submission(&self, submission: &Submission) -> Option<&SubmissionResult> {
        self.submissions
            .iter()
            .find(|&s| s.submission.eq(submission))
    }

    pub fn save_to_cache(&self) -> Result<()> {
        let cache_path = Self::cache_path(self.year, self.day);
        let cache_dir = cache_path.parent().unwrap();
        if !cache_path.exists() {
            std::fs::create_dir_all(cache_dir).chain_err(|| {
                ErrorKind::CacheFailure(format!(
                    "Failed to create cache directory: {}",
                    cache_dir.display()
                ))
            })?;
        }
        let serialized = serde_cbor::to_vec(&self).chain_err(|| {
            ErrorKind::CacheFailure(format!(
                "Failed to serialize cache file: {}",
                cache_path.display()
            ))
        })?;
        std::fs::write(&cache_path, serialized).chain_err(|| {
            ErrorKind::CacheFailure(format!(
                "Failed to write cache file: {}",
                cache_path.display()
            ))
        })?;

        Ok(())
    }

    pub fn clear() -> Result<()> {
        let cache_dir = Configuration::get_project_directories()
            .cache_dir()
            .join("submissions");
        if cache_dir.exists() {
            std::fs::remove_dir_all(&cache_dir).chain_err(|| {
                ErrorKind::CacheFailure(format!(
                    "Failed to remove cache directory: {}",
                    cache_dir.display()
                ))
            })?
        }
        Ok(())
    }

    fn cache_path(year: u16, day: u8) -> std::path::PathBuf {
        Configuration::get_project_directories()
            .cache_dir()
            .join("submissions")
            .join(format!("{}-{}", year, day))
    }
}

#[cfg(test)]
mod tests {
    use super::SubmissionHistory;
    use crate::aoc_domain::{RiddlePart, Submission, SubmissionResult, SubmissionStatus};

    #[test]
    fn can_add_submission() {
        let submission = Submission::new(RiddlePart::One, "7".to_string(), 2020, 1);
        let submission_result = SubmissionResult::new(
            submission,
            SubmissionStatus::Correct,
            concat!(
                "That's the right answer! You are one gold star closer to saving your vacation.",
                " You got rank 1 on this star's leaderboard. [Return to Day 1]"
            )
            .to_string(),
            chrono::Utc::now(),
            7,
        );
        let mut submission_history = SubmissionHistory::new(2020, 1);
        submission_history.add(submission_result);
        assert_eq!(submission_history.submissions.len(), 1);
    }

    #[test]
    fn previously_added_submission_result_can_be_retrieved() {
        let submission = Submission::new(RiddlePart::One, "7".to_string(), 2020, 1);
        let submission_result = SubmissionResult::new(
            submission,
            SubmissionStatus::Correct,
            concat!(
                "That's the right answer! You are one gold star closer to saving your vacation.",
                " You got rank 1 on this star's leaderboard. [Return to Day 1]"
            )
            .to_string(),
            chrono::Utc::now(),
            7,
        );
        let mut submission_history = SubmissionHistory::new(2020, 1);
        submission_history.add(submission_result);
        let new_submission = Submission::new(RiddlePart::One, "7".to_string(), 2020, 1);
        let retrieved_submission_result =
            submission_history.get_result_for_submission(&new_submission);
        assert_eq!(
            retrieved_submission_result.unwrap().submission,
            new_submission
        );
    }

    #[test]
    fn get_result_for_submission_returns_none_for_a_new_submission() {
        let submission = Submission::new(RiddlePart::One, "7".to_string(), 2020, 1);
        let submission_history = SubmissionHistory::new(2020, 1);
        assert_eq!(
            submission_history.get_result_for_submission(&submission),
            None
        );
    }

    #[test]
    fn can_submit_returns_false_if_submitted_too_soon() {
        let submission_result = SubmissionResult::new(
            Submission::new(RiddlePart::One, "7".to_string(), 2020, 1),
            SubmissionStatus::Correct,
            concat!(
                "That's the right answer! You are one gold star closer to saving your vacation.",
                " You got rank 1 on this star's leaderboard. [Return to Day 1]"
            )
            .to_string(),
            chrono::Utc::now(),
            7,
        );
        let mut submission_history = SubmissionHistory::new(2020, 1);
        submission_history.add(submission_result);
        assert!(!submission_history.can_submit(chrono::Utc::now()));
    }

    #[test]
    fn can_submit_if_there_are_no_submissions() {
        let submission_history = SubmissionHistory::new(2020, 1);
        assert!(submission_history.can_submit(chrono::Utc::now()));
    }

    #[test]
    fn test_wait_time() {
        let now = chrono::Utc::now();
        let submission_result = SubmissionResult::new(
            Submission::new(RiddlePart::One, "7".to_string(), 2020, 1),
            SubmissionStatus::Correct,
            concat!(
                "That's the right answer! You are one gold star closer to saving your vacation.",
                " You got rank 1 on this star's leaderboard. [Return to Day 1]"
            )
            .to_string(),
            now,
            7,
        );
        let mut submission_history = SubmissionHistory::new(2020, 1);
        submission_history.add(submission_result);
        assert_eq!(
            submission_history.wait_time(now + chrono::Duration::minutes(4)),
            Some(chrono::Duration::minutes(3))
        );

        assert_eq!(
            submission_history.wait_time(now + chrono::Duration::minutes(8)),
            None
        )
    }
}
