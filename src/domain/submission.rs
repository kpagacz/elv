use super::riddle_part::RiddlePart;

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
pub struct Submission {
    pub part: RiddlePart,
    pub answer: String,
    pub year: usize,
    pub day: usize,
}

impl Submission {
    pub fn new(part: RiddlePart, answer: String, year: usize, day: usize) -> Self {
        Submission {
            part,
            answer,
            year,
            day,
        }
    }
}
