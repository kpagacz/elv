use crate::{
    domain::{
        description::Description, submission::Submission, submission_result::SubmissionResult,
    },
    infrastructure::aoc_api::aoc_client_impl::InputResponse,
};

use super::errors::AocClientError;

pub trait AocClient {
    fn submit_answer(&self, submission: Submission) -> Result<SubmissionResult, AocClientError>;
    fn get_description<Desc>(&self, year: i32, day: i32) -> Result<Desc, AocClientError>
    where
        Desc: Description + TryFrom<reqwest::blocking::Response>;
    fn get_input(&self, year: i32, day: i32) -> InputResponse;
}
