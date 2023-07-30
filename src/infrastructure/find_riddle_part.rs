use crate::domain::RiddlePart;

pub trait FindRiddlePart {
    fn find(&self, year: i32, day: i32) -> Result<RiddlePart, anyhow::Error>;
}
