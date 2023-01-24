use std::io::Read;

use crate::domain::{errors::*, RiddlePart, Submission, SubmissionResult, SubmissionStatus, Description};
use crate::infrastructure::{Configuration, HttpDescription};
use error_chain::bail;
use reqwest::header::{CONTENT_TYPE, ORIGIN};

const AOC_URL: &str = "https://adventofcode.com";

#[derive(Debug)]
pub struct AocApi<'a> {
    http_client: reqwest::blocking::Client,
    configuration: &'a Configuration,
}

impl<'a> AocApi<'a> {
    pub fn new(configuration: &'a Configuration) -> AocApi<'a> {
        Self {
            http_client: Self::prepare_http_client(configuration),
            configuration,
        }
    }

    pub fn get_input(&self, year: &u16, day: &u8) -> InputResponse {
        let url = match reqwest::Url::parse(&format!("{}/{}/day/{}/input", AOC_URL, year, day)) {
            Ok(url) => url,
            Err(_) => {
                return InputResponse::new(
                    "Failed to parse the URL. Are you sure your day and year are correct?"
                        .to_string(),
                    ResponseStatus::Error,
                )
            }
        };
        let mut response = match self.http_client.get(url).send() {
            Ok(response) => response,
            Err(_) => return InputResponse::failed(),
        };
        if response.status() != reqwest::StatusCode::OK {
            return InputResponse::new(
                "Got a non-200 status code from the server. Is your token up to date?".to_owned(),
                ResponseStatus::Error,
            );
        }
        let mut body = String::new();
        if response.read_to_string(&mut body).is_err() {
            return InputResponse::new(
                "Failed to read the response body".to_owned(),
                ResponseStatus::Error,
            );
        }
        if body.starts_with("Please don't repeatedly request this") {
            return InputResponse::new(
                "You have to wait for the input to be available".to_owned(),
                ResponseStatus::TooSoon,
            );
        }
        InputResponse::new(body, ResponseStatus::Ok)
    }

    pub fn submit_answer(&self, submission: Submission) -> Result<SubmissionResult> {
        let url = reqwest::Url::parse(&format!(
            "{}/{}/day/{}/answer",
            AOC_URL, submission.year, submission.day
        ))?;
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
            .send()?;
        if !response.status().is_success() {
            bail!(
                "Failed to submit the answer. Status code: {}",
                response.status()
            );
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
    pub fn get_description(&self, year: &u16, day: &u8) -> Result<HttpDescription> {
        let url = reqwest::Url::parse(&format!("{}/{}/day/{}", AOC_URL, year, day))
            .chain_err(|| "Failed to form the url for the description")?;
        Ok(self
            .http_client
            .get(url)
            .send()
            .chain_err(|| "Failed to get the response from the AoC server")?
            .try_into()?)
    }

    fn prepare_http_client(configuration: &Configuration) -> reqwest::blocking::Client {
        let cookie = format!("session={}", configuration.aoc.token);
        let url = AOC_URL.parse::<reqwest::Url>().expect("Invalid URL");
        let jar = reqwest::cookie::Jar::default();
        jar.add_cookie_str(&cookie, &url);

        reqwest::blocking::Client::builder()
            .cookie_provider(std::sync::Arc::new(jar))
            .user_agent(Self::aoc_elf_user_agent())
            .build()
            .chain_err(|| "Failed to create HTTP client")
            .unwrap()
    }

    fn aoc_elf_user_agent() -> String {
        let pkg_name: &str = env!("CARGO_PKG_NAME");
        let pkg_version: &str = env!("CARGO_PKG_VERSION");

        format!(
            "{}/{} (+{} author:{})",
            pkg_name, pkg_version, "https://github.com/kpagacz/elv", "konrad.pagacz@gmail.com"
        )
    }

    fn extract_wait_time_from_message(message: &str) -> std::time::Duration {
        let please_wait_marker = "lease wait ";
        let please_wait_position = match message.find(please_wait_marker) {
            Some(position) => position,
            None => return std::time::Duration::new(0, 0),
        };
        let minutes_position = please_wait_position + please_wait_marker.len();
        let next_space_position = message[minutes_position..].find(' ').unwrap();
        let minutes = &message[minutes_position..minutes_position + next_space_position];
        if minutes == "one" {
            std::time::Duration::from_secs(60)
        } else {
            std::time::Duration::from_secs(60 * minutes.parse::<u64>().unwrap_or(0))
        }
    }

    fn get_aoc_answer_selector() -> scraper::Selector {
        scraper::Selector::parse("main > article > p").unwrap()
    }

    fn parse_submission_answer_body(self: &Self, body: &str) -> Result<String> {
        let document = scraper::Html::parse_document(body);
        let answer = document
            .select(&Self::get_aoc_answer_selector())
            .next()
            .chain_err(|| "Failed to parse the answer")?;
        let answer_text = html2text::from_read(
            answer.text().collect::<Vec<_>>().join("").as_bytes(),
            self.configuration.cli.output_width,
        );
        Ok(answer_text)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ResponseStatus {
    Ok,
    TooSoon,
    Error,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InputResponse {
    pub body: String,
    pub status: ResponseStatus,
}

impl InputResponse {
    pub fn new(body: String, status: ResponseStatus) -> Self {
        Self { body, status }
    }

    pub fn failed() -> Self {
        Self::new("Failed to get input".to_string(), ResponseStatus::Error)
    }
}

#[cfg(test)]
mod tests {
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
        let api = AocApi::new(&configuration);
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
        let api = AocApi::new(&configuration);
        let message = api.parse_submission_answer_body(body).unwrap();
        assert_eq!(message, concat!(
            "That's not the right answer. If you're stuck, make sure you're using the full input data; there are also some general\n",
            "tips on the about page, or you can ask for hints on the subreddit. ",
            "Because you have guessed incorrectly 7 times on this\npuzzle, please ",
            "wait 10 minutes before trying again. (You guessed 0.) [Return to Day 1]\n"))
    }
}
