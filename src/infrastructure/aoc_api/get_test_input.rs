use crate::domain::ports::get_test_input::GetExampleInput;
use anyhow::Result;

use super::AocApi;

impl GetExampleInput for AocApi {
    fn get_example_input(&mut self, day: usize, year: usize) -> Result<String> {
        todo!()
    }
}
