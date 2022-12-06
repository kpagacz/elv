use crate::aoc_api::AocApi;
use crate::aoc_domain::Submission;
use crate::configuration::Configuration;
use crate::submission_history::SubmissionHistory;

const CACHE_ERROR_MESSAGE: &str = concat!(
    "Could not save the submission result to the cache and the application will not ",
    "have any memory of this submission.",
    " Please check your permissions.",
    " If you are using Windows, please run the program as administrator.",
    " Be aware that the cache is used to prevent spamming the Advent of Code servers.",
    " If you spam the servers, your IP might be banned from the Advent of Code servers.",
);

#[derive(Debug, Default)]
pub struct Driver {
    configuration: Configuration,
}

impl Driver {
    pub fn input(year: u32, day: u8) {
        let aoc_api = AocApi::default();
        let input = aoc_api.get_input(&year, &day).expect(concat!(
            "Could not get input.",
            " Please check your internet connection and make sure you supply an Advent of Code ",
            "session token to this application's configuration."
        ));
        println!("{}", input);
    }

    pub fn submit_answer(
        &self,
        year: u16,
        day: u8,
        part: crate::aoc_domain::RiddlePart,
        answer: String,
    ) {
        let aoc_api = AocApi::default();
        let mut cache: Option<SubmissionHistory> = match SubmissionHistory::from_cache(year, day) {
            Ok(c) => Some(c),
            Err(e) => {
                println!("Error: {}", e);
                println!("The application will not have any memory of this submission.");
                None
            }
        };
        let submission = Submission::new(part, answer, year, day);
        if let Some(ref res) = cache {
            if let Some(submission_result) = res.get_result_for_submission(&submission) {
                println!("Your submission result:\n{:?}", submission_result.message);
                return;
            }
        }

        match aoc_api.submit_answer(submission) {
            Ok(res) => {
                println!("Your submission result:\n{:?}", res.message);
                if let Some(ref mut cache) = cache {
                    cache.add(res);
                    cache.save_to_cache().expect(CACHE_ERROR_MESSAGE);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
