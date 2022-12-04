use crate::{
    aoc_domain::{Submission, SubmissionResult},
    configuration::get_project_directories,
};
use chrono::Datelike;
use serde::{Deserialize, Serialize};

use crate::errors::*;

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

    pub fn from(submissions: Vec<SubmissionResult>, year: u16, day: u8) -> Self {
        SubmissionHistory {
            submissions,
            year,
            day,
        }
    }

    pub fn from_cache(year: u16, day: u8) -> Result<Self> {
        let cache_path = Self::cache_path(year, day);
        if !cache_path.exists() {
            return Err(ErrorKind::CacheFailure(format!(
                "Cache file not found: {}",
                cache_path.display()
            ))
            .into());
        }
        let content = std::fs::read(cache_path)?;
        serde_cbor::from_slice::<SubmissionHistory>(&content).or(Err(ErrorKind::CacheFailure(
            "Failed to deserialize cache".to_string(),
        )
        .into()))
    }
    pub fn add(&mut self, submission: SubmissionResult) {
        self.submissions.push(submission);
    }

    pub fn can_submit(&self, submission: &Submission) -> bool {
        return false;
    }

    pub fn previously_submitted(&self, submission: &Submission) -> bool {
        self.submissions.iter().any(|s| s.submission.eq(submission))
    }

    pub fn get_submission_result(&self, submission: &Submission) -> Option<&SubmissionResult> {
        self.submissions
            .iter()
            .find(|&s| s.submission.eq(submission))
    }

    pub fn save_to_cache(&self) -> Result<()> {
        let cache_path = Self::cache_path(self.year, self.day);
        let cache_dir = cache_path.parent().unwrap();
        if !cache_path.exists() {
            std::fs::create_dir_all(cache_dir)?;
        }

        let serialized = serde_cbor::to_vec(&self)?;
        std::fs::write(cache_path, serialized)?;

        Ok(())
    }

    fn cache_path(year: u16, day: u8) -> std::path::PathBuf {
        get_project_directories()
            .cache_dir()
            .join(format!("{}-{}", year, day))
    }
}
