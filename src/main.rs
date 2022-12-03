mod aoc_domain;
mod aoc_interface;
mod errors;
use aoc_domain::{RiddlePart, Submission};
use std::fs;

fn main() {
    let client = aoc_interface::prepare_http_client();
    // aoc_interface::get_input(&1u8, &client).unwrap();
    // aoc_interface::submit_answer(
    //     Submission::new(RiddlePart::One, "7".to_string(), 2020, 1),
    //     &client,
    // )
    // .unwrap();
    let submission_result = aoc_interface::submit_answer(
        Submission::new(RiddlePart::Two, "85491920".to_string(), 2020, 1),
        &client,
    )
    .unwrap();
    println!("{:#?}", submission_result);
}
