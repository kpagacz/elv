use anyhow::Result;

pub trait GetInput {
    fn get_input(&mut self, day: usize, year: usize) -> Result<String>;
}
