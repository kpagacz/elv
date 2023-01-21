use super::{Submission, SubmissionStatus};

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
pub struct SubmissionResult {
    pub submission: Submission,
    pub status: SubmissionStatus,
    pub message: String,

    #[serde(with = "chrono::serde::ts_seconds")]
    pub submitted_at: chrono::DateTime<chrono::Utc>,

    pub wait_time: std::time::Duration,
}

impl SubmissionResult {
    pub fn new(
        submission: Submission,
        status: SubmissionStatus,
        message: String,
        submitted_at: chrono::DateTime<chrono::Utc>,
        wait_time: std::time::Duration,
    ) -> Self {
        SubmissionResult {
            submission,
            status,
            message,
            submitted_at,
            wait_time,
        }
    }
}
