use crate::domain::riddle_part::RiddlePart;

pub trait FindRiddlePart {
    fn find_unsolved_part(&self, year: usize, day: usize) -> Result<RiddlePart, anyhow::Error>;
}
