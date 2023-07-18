use crate::domain::Description;
use crate::domain::{Submission, SubmissionResult};
use crate::infrastructure::aoc_api::aoc_client_impl::InputResponse;

use super::errors::AocClientError;

pub trait AocClient {
    fn submit_answer(&self, submission: Submission) -> Result<SubmissionResult, AocClientError>;
    fn get_description<Desc>(&self, year: &u16, day: &u8) -> Result<Desc, AocClientError>
    where
        Desc: Description + TryFrom<reqwest::blocking::Response>;
    fn get_input(&self, year: &u16, day: &u8) -> InputResponse;
}
