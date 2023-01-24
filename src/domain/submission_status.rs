#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
pub enum SubmissionStatus {
    Correct,
    Incorrect,
    Unknown,
    TooSoon,
    WrongLevel,
}
