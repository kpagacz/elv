use anyhow::Result;

pub trait GetExampleInput {
    fn get_example_input(&mut self, day: usize, year: usize) -> Result<String>;
}
