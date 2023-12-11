use anyhow::Result;

pub trait GetExampleInput {
    fn get_example_input(&self, day: usize, year: usize) -> Result<String>;
}
