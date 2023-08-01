use crate::domain::RiddlePart;

pub trait FindRiddlePart {
    fn find_unsolved_part(&self, year: i32, day: i32) -> Result<RiddlePart, anyhow::Error>;
}
