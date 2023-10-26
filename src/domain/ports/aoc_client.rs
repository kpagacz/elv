use crate::domain::{
    description::Description, submission::Submission, submission_result::SubmissionResult,
};

use super::errors::AocClientError;

pub trait AocClient {
    fn submit_answer(&self, submission: Submission) -> Result<SubmissionResult, AocClientError>;
    fn get_description<Desc>(&self, year: usize, day: usize) -> Result<Desc, AocClientError>
    where
        Desc: Description + TryFrom<reqwest::blocking::Response>;
}
