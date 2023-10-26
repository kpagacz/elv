use anyhow::Result;

pub trait GetInput {
    fn get_input(&self, day: usize, year: usize) -> Result<String>;
}
