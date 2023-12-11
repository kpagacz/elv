use crate::domain::{
    description::Description, submission::Submission, submission_result::SubmissionResult,
};
use anyhow::Result;

use super::errors::AocClientError;

pub trait AocClient {
    type Desc;

    fn submit_answer(&self, submission: Submission) -> Result<SubmissionResult, AocClientError>;
    fn get_description(&self, year: usize, day: usize) -> Result<Self::Desc>
    where
        Self::Desc: Description;
}
