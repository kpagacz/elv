mod aoc_domain;
mod aoc_interface;
mod configuration;
mod errors;
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
}
