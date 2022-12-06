use std::fmt::Display;

use clap::ValueEnum;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ValueEnum)]
pub enum RiddlePart {
    One = 1,
    Two,
}

impl Display for RiddlePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiddlePart::One => write!(f, "one"),
            RiddlePart::Two => write!(f, "two"),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Submission {
    pub part: RiddlePart,
    pub answer: String,
    pub year: u16,
    pub day: u8,
}

impl Submission {
    pub fn new(part: RiddlePart, answer: String, year: u16, day: u8) -> Self {
        Submission {
            part,
            answer,
            year,
            day,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SubmissionStatus {
    Correct,
    Incorrect,
    Unknown,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmissionResult {
    pub submission: Submission,
    pub status: SubmissionStatus,
    pub message: String,

    #[serde(with = "chrono::serde::ts_seconds")]
    pub submitted_at: chrono::DateTime<chrono::Utc>,

    pub wait_minutes: i64,
}

impl SubmissionResult {
    pub fn new(submission: Submission, status: SubmissionStatus, message: String, submitted_at: chrono::DateTime<chrono::Utc>, wait_minutes: i64) -> Self {
        SubmissionResult {
            submission,
            status,
            message,
            submitted_at,
            wait_minutes,
        }
    }
}
