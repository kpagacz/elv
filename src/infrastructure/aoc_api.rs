use super::configuration::Configuration;
use anyhow::Result;

const AOC_URL: &str = "https://adventofcode.com";

#[derive(Debug)]
pub struct AocApi {
    http_client: reqwest::blocking::Client,
    configuration: Configuration,
    description: Option<(usize, usize, Result<String>)>,
}

mod aoc_api_impl;
pub mod aoc_client_impl;
pub mod find_riddle_part_impl;
pub mod get_input_impl;
pub mod get_leaderboard_impl;
pub mod get_private_leaderboard_impl;
pub mod get_stars_impl;
