mod aoc_domain;
mod aoc_interface;
mod configuration;
mod errors;
mod submission_history;

use crate::aoc_domain::*;
use crate::submission_history::SubmissionHistory;
use configuration::get_config;

fn main() {
    let configuration = get_config();
    let client = aoc_interface::prepare_http_client(&configuration);
    // aoc_interface::get_input(&1u8, &client).unwrap();
    // aoc_interface::submit_answer(
    //     Submission::new(RiddlePart::One, "7".to_string(), 2020, 1),
    //     &client,
    // )
    // .unwrap();
    // let submission_result = aoc_interface::submit_answer(
    //     Submission::new(RiddlePart::Two, "85491920".to_string(), 2020, 1),
    //     &client,
    // )
    // .unwrap();
    // println!("{:#?}", submission_result);
    let config = get_config();
    println!("{:#?}", config);

    println!("Testing caching");
    let submission_history = SubmissionHistory::from(
        vec![
            SubmissionResult::new(
                Submission::new(RiddlePart::One, "7".to_string(), 2020, 1),
                SubmissionStatus::Correct,
                "That's the right answer! You are one gold star closer to saving your vacation. You got rank 1 on this star's leaderboard. [Return to Day 1]".to_string(),
            ),
            SubmissionResult::new(
                Submission::new(RiddlePart::Two, "85491920".to_string(), 2020, 1),
                SubmissionStatus::Correct,
                "That's the right answer! You are one gold star closer to saving your vacation. You got rank 1 on this star's leaderboard. [Return to Day 1]".to_string(),
            ),
        ],
        2020,
        1,
    );

    println!(
        "{}",
        configuration::get_project_directories()
            .cache_dir()
            .to_str()
            .unwrap()
    );
    println!("{:#?}", submission_history);
    submission_history.save_to_cache().unwrap();

    let submission_history = SubmissionHistory::from_cache(2020, 1).unwrap();
    println!("{:#?}", submission_history);
}
