#[derive(Debug, PartialEq)]
pub enum RiddlePart {
    One,
    Two,
}

#[derive(Debug)]
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

#[derive(Debug, PartialEq)]
pub enum SubmissionStatus {
    Correct,
    Incorrect,
    Unknown,
}

#[derive(Debug)]
pub struct SubmissionResult {
    pub submission: Submission,
    pub status: SubmissionStatus,
    pub message: String,
}

impl SubmissionResult {
    pub fn new(submission: Submission, status: SubmissionStatus, message: String) -> Self {
        SubmissionResult {
            submission,
            status,
            message,
        }
    }
}
