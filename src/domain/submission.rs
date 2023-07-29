use super::RiddlePart;

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
pub struct Submission {
    pub part: RiddlePart,
    pub answer: String,
    pub year: i32,
    pub day: i32,
}

impl Submission {
    pub fn new(part: RiddlePart, answer: String, year: i32, day: i32) -> Self {
        Submission {
            part,
            answer,
            year,
            day,
        }
    }
}
