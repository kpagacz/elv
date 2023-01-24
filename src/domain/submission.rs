use super::RiddlePart;

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
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
