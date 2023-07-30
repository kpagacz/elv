use super::configuration::Configuration;

const AOC_URL: &str = "https://adventofcode.com";

#[derive(Debug)]
pub struct AocApi {
    http_client: reqwest::blocking::Client,
    configuration: Configuration,
}

mod aoc_api_impl;
pub mod aoc_client_impl;
pub mod find_riddle_part_impl;
pub mod get_leaderboard_impl;
