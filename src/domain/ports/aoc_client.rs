use crate::domain::errors::*;
use crate::domain::Description;
use crate::domain::{Submission, SubmissionResult};
use crate::infrastructure::http::aoc_api::InputResponse;

pub trait AocClient {
    fn submit_answer(&self, submission: Submission) -> Result<SubmissionResult>;
    fn get_description<Desc>(&self, year: &u16, day: &u8) -> Result<Desc>
    where
        Desc: Description + TryFrom<reqwest::blocking::Response>;
    fn get_input(&self, year: &u16, day: &u8) -> InputResponse;
}
