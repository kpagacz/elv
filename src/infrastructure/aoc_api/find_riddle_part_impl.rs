use crate::domain::{ports::aoc_client::AocClient, riddle_part::RiddlePart};
use crate::infrastructure::find_riddle_part::FindRiddlePart;

use super::AocApi;

impl FindRiddlePart for AocApi {
    fn find_unsolved_part(&self, year: usize, day: usize) -> Result<RiddlePart, anyhow::Error> {
        let description = Self::get_description(self, year, day)?;
        match (description.part_one_answer(), description.part_two_answer()) {
            (None, _) => Ok(RiddlePart::One),
            (Some(_), None) => Ok(RiddlePart::Two),
            (_, _) => anyhow::bail!("Both parts answered"),
        }
    }
}
