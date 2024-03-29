use super::{AocApi, AOC_URL};
use crate::domain::{
    ports::{aoc_client::AocClient, errors::AocClientError},
    riddle_part::RiddlePart,
    submission::Submission,
    submission_result::SubmissionResult,
    submission_status::SubmissionStatus,
};
use reqwest::header::{CONTENT_TYPE, ORIGIN};
use std::io::Read;

impl AocClient for AocApi {
    fn submit_answer(&self, submission: Submission) -> Result<SubmissionResult, AocClientError> {
        let url = reqwest::Url::parse(&format!(
            "{}/{}/day/{}/answer",
            AOC_URL, submission.year, submission.day
        ))
        .map_err(|e| AocClientError::SubmitAnswerError(e.to_string()))?;
        let mut response = self
            .http_client
            .post(url)
            .body(format!(
                "level={}&answer={}",
                match submission.part {
                    RiddlePart::One => 1,
                    RiddlePart::Two => 2,
                },
                submission.answer
            ))
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(ORIGIN, "https://adventofcode.com")
            .header(
                "Referer",
                format!(
                    "https://adventofcode.com/{}/day/{}",
                    submission.year, submission.day
                ),
            )
            .send()
            .map_err(|e| AocClientError::SubmitAnswerError(e.to_string()))?;
        if !response.status().is_success() {
            return Err(AocClientError::SubmitAnswerError(format!(
                "Status code: {}",
                response.status()
            )));
        }

        let mut body = String::new();
        response.read_to_string(&mut body)?;

        let message = self.parse_submission_answer_body(&body)?;
        let submission_status = if message.starts_with("That's the right answer!") {
            SubmissionStatus::Correct
        } else if message.starts_with("You gave an answer too recently") {
            SubmissionStatus::TooSoon
        } else if message.starts_with("You don't seem to be solving the right level") {
            SubmissionStatus::WrongLevel
        } else {
            SubmissionStatus::Incorrect
        };

        let mut wait_time = std::time::Duration::new(0, 0);
        if submission_status == SubmissionStatus::Incorrect
            || submission_status == SubmissionStatus::TooSoon
        {
            wait_time = Self::extract_wait_time_from_message(&message);
        }

        Ok(SubmissionResult::new(
            submission,
            submission_status,
            message,
            chrono::Utc::now(),
            wait_time,
        ))
    }

    /// Queries the Advent of Code website for the description of a riddle
    /// for a given day and year and returns it as a formatted string.
    fn get_description<HttpDescription: std::convert::TryFrom<reqwest::blocking::Response>>(
        &self,
        year: usize,
        day: usize,
    ) -> Result<HttpDescription, AocClientError> {
        let url = reqwest::Url::parse(&format!("{}/{}/day/{}", AOC_URL, year, day))?;
        self.http_client
            .get(url)
            .send()?
            .try_into()
            .map_err(|_e| AocClientError::GetDescriptionError)
    }
}

#[cfg(test)]
mod tests {
    use crate::Configuration;

    use super::*;

    #[test]
    fn extraction_of_wait_time_from_message1() {
        let message = "That's not the right answer; your answer is too low. Please wait one minute and try again (you guessed 1).";
        let wait_time = AocApi::extract_wait_time_from_message(message);
        assert_eq!(wait_time, std::time::Duration::from_secs(60));
    }

    #[test]
    fn extraction_of_wait_time_from_message2() {
        let message = "That's not the right answer; your answer is too low. Please wait 2 minutes and try again (you guessed 1).";
        let wait_time = AocApi::extract_wait_time_from_message(message);
        assert_eq!(wait_time, std::time::Duration::from_secs(2 * 60));
    }

    #[test]
    fn test_parse_submission_answer_body() {
        let body = r#"
        <main>
<article><p>That's the right answer!  You
 are <span class="day-success">one gold star</span> closer to saving your
 vacation. <a href="/2020/day/1#part2">[Continue to Part Two]</a></p></article>
</main>"#;

        let configuration = Configuration::default();
        let api_client = AocApi::prepare_http_client(&configuration);
        let api = AocApi::new(api_client, configuration);
        let message = api.parse_submission_answer_body(body).unwrap();
        assert_eq!(message, "That's the right answer! You are one gold star closer to saving your vacation. [Continue to Part Two]\n");
    }

    #[test]
    fn test_parse_submission_answer_body2() {
        let body = r#"
        <main>
        <article><p>That's not the right answer.
        If you're stuck, make sure you're using the full input data; there are also some general tips on the
        <a href="/2020/about">about page</a>, or you can ask for hints on the <a href="https://www.reddit.com/r/adventofcode/"
        target="_blank">subreddit</a>.  Because you have guessed incorrectly 7 times on this puzzle,
        please wait 10 minutes before trying again. (You guessed <span style="white-space:nowrap;"><code>0</code>.)</span>
        <a href="/2020/day/1">[Return to Day 1]</a></p></article>
        </main>
        "#;

        let configuration = Configuration::default();
        let http_client = AocApi::prepare_http_client(&configuration);
        let api = AocApi::new(http_client, configuration);
        let message = api.parse_submission_answer_body(body).unwrap();
        assert_eq!(message, concat!(
            "That's not the right answer. If you're stuck, make sure you're using the full input data; there are also some general\n",
            "tips on the about page, or you can ask for hints on the subreddit. ",
            "Because you have guessed incorrectly 7 times on this\npuzzle, please ",
            "wait 10 minutes before trying again. (You guessed 0.) [Return to Day 1]\n"))
    }
}
