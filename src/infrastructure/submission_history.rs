use thiserror::Error;

use crate::domain::{
    riddle_part::RiddlePart, submission::Submission, submission_result::SubmissionResult,
    submission_status::SubmissionStatus,
};

use super::configuration::Configuration;

#[derive(Error, Debug)]
pub enum SubmissionHistoryError {
    #[error("Error loading the data from the submission history cache: {}", 0)]
    Load(String),
    #[error("Error saving data to the submission history cache: {}", 0)]
    Save(String),
    #[error("Error clearing the submission history cache: {}", 0)]
    Clear(String),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SubmissionHistory {
    submissions: Vec<SubmissionResult>,
    year: usize,
    day: usize,
}

impl SubmissionHistory {
    pub fn new(year: usize, day: usize) -> Self {
        SubmissionHistory {
            submissions: Vec::new(),
            year,
            day,
        }
    }

    pub fn correct_submission(&self, part: &RiddlePart) -> Option<&SubmissionResult> {
        self.submissions
            .iter()
            .find(|s| s.submission.part == *part && s.status == SubmissionStatus::Correct)
    }

    pub fn from_cache(year: usize, day: usize) -> Result<Self, SubmissionHistoryError> {
        let cache_path = Self::cache_path(year, day);
        if !cache_path.exists() {
            Self::new(year, day).save_to_cache()?;
        }
        let content = std::fs::read(&cache_path).map_err(|_| {
            SubmissionHistoryError::Load(format!(
                "Failed to read cache file: {}",
                cache_path.display()
            ))
        })?;
        serde_cbor::from_slice::<SubmissionHistory>(&content).map_err(|_| {
            SubmissionHistoryError::Load(format!(
                "Failed to deserialize cache file: {}",
                cache_path.display()
            ))
        })
    }

    pub fn add(&mut self, submission: SubmissionResult) {
        self.submissions.push(submission);
    }

    pub fn wait_time(
        &self,
        now: &chrono::DateTime<chrono::Utc>,
        part: &RiddlePart,
    ) -> Option<chrono::Duration> {
        match self.last_submission(part) {
            Some(last) => {
                let wait_time =
                    last.submitted_at + chrono::Duration::from_std(last.wait_time).unwrap() - *now;
                if wait_time.num_seconds() > 0 {
                    Some(wait_time)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn get_result_for_submission(&self, submission: &Submission) -> Option<&SubmissionResult> {
        self.submissions
            .iter()
            .find(|&s| s.submission.eq(submission))
    }

    pub fn save_to_cache(&self) -> Result<(), SubmissionHistoryError> {
        let cache_path = Self::cache_path(self.year, self.day);
        let cache_dir = cache_path.parent().unwrap();
        if !cache_path.exists() {
            std::fs::create_dir_all(cache_dir).map_err(|_| {
                SubmissionHistoryError::Save(format!(
                    "Failed to create cache directory: {}",
                    cache_dir.display()
                ))
            })?;
        }
        let serialized = serde_cbor::to_vec(&self).map_err(|_| {
            SubmissionHistoryError::Save(format!(
                "Failed to serialize cache file: {}",
                cache_path.display()
            ))
        })?;
        std::fs::write(&cache_path, serialized).map_err(|_| {
            SubmissionHistoryError::Save(format!(
                "Failed to write cache file: {}",
                cache_path.display()
            ))
        })?;

        Ok(())
    }

    pub fn clear() -> Result<(), SubmissionHistoryError> {
        let cache_dir = Configuration::get_project_directories()
            .cache_dir()
            .join("submissions");
        if cache_dir.exists() {
            std::fs::remove_dir_all(&cache_dir).map_err(|_| {
                SubmissionHistoryError::Clear(format!(
                    "Failed to remove cache directory: {}",
                    cache_dir.display()
                ))
            })?
        }
        Ok(())
    }

    fn cache_path(year: usize, day: usize) -> std::path::PathBuf {
        Configuration::get_project_directories()
            .cache_dir()
            .join("submissions")
            .join(format!("{}-{}", year, day))
    }

    fn last_submission(&self, part: &RiddlePart) -> Option<&SubmissionResult> {
        self.submissions
            .iter()
            .filter(|s| s.submission.part == *part)
            .last()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        riddle_part::RiddlePart, submission::Submission, submission_result::SubmissionResult,
        submission_status::SubmissionStatus,
    };

    use super::SubmissionHistory;

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
            std::time::Duration::from_secs(7 * 60),
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
            std::time::Duration::from_secs(7 * 60),
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
            std::time::Duration::from_secs(7 * 60),
        );
        let mut submission_history = SubmissionHistory::new(2020, 1);
        submission_history.add(submission_result);
        assert_eq!(
            submission_history.wait_time(&(now + chrono::Duration::minutes(4)), &RiddlePart::One),
            Some(chrono::Duration::minutes(3))
        );

        assert_eq!(
            submission_history.wait_time(&(now + chrono::Duration::minutes(8)), &RiddlePart::One),
            None
        )
    }

    #[test]
    fn correct_submission() {
        let submission_result = SubmissionResult::new(
            Submission::new(RiddlePart::One, "7".to_string(), 2020, 1),
            SubmissionStatus::Correct,
            concat!(
                "That's the right answer! You are one gold star closer to saving your vacation.",
                " You got rank 1 on this star's leaderboard. [Return to Day 1]"
            )
            .to_string(),
            chrono::Utc::now(),
            std::time::Duration::from_secs(7 * 60),
        );
        let mut submission_history = SubmissionHistory::new(2020, 1);
        submission_history.add(submission_result.clone());
        let correct_submission = submission_history.correct_submission(&RiddlePart::One);
        assert!(correct_submission.is_some());
        assert_eq!(correct_submission.unwrap(), &submission_result);
    }
}
