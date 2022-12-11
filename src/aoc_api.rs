use crate::aoc_domain::{RiddlePart, Submission, SubmissionResult, SubmissionStatus};
use crate::configuration::Configuration;
use crate::errors::*;
use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, ORIGIN};
use reqwest::StatusCode;
use reqwest::{cookie::Jar, Url};
use scraper::{Html, Selector};
use std::io::Read;
use std::sync::Arc;

const AOC_URL: &str = "https://adventofcode.com";

#[derive(Debug, Default)]
pub struct AocApi {
    http_client: Client,
}

impl AocApi {
    pub fn new(configuration: &Configuration) -> Self {
        Self {
            http_client: Self::prepare_http_client(configuration),
        }
    }

    pub fn default() -> Self {
        Self {
            http_client: Self::prepare_http_client(&Configuration::new()),
        }
    }

    pub fn get_input(&self, year: &u16, day: &u8) -> InputResponse {
        let url = match Url::parse(&format!("{}/{}/day/{}/input", AOC_URL, year, day)) {
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
            Err(_) => return Self::failed_input_request_response(),
        };
        if response.status() != StatusCode::OK {
            return Self::failed_input_request_response();
        }
        let mut body = String::new();
        if response.read_to_string(&mut body).is_err() {
            return Self::failed_input_request_response();
        }
        if body.starts_with("Please don't repeatedly request this") {
            return InputResponse::new(
                "You have to wait for the input to be available".to_string(),
                ResponseStatus::TooSoon,
            );
        }
        InputResponse::new(body, ResponseStatus::Ok)
    }

    pub fn submit_answer(&self, submission: Submission) -> Result<SubmissionResult> {
        let url = Url::parse(&format!(
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
        let mut body = String::new();
        response.read_to_string(&mut body)?;

        let message = Self::parse_submission_answer_body(&body)?;
        let submission_status = if message.starts_with("That's the right answer!") {
            SubmissionStatus::Correct
        } else if message.starts_with("You gave an answer too recently") {
            SubmissionStatus::TooSoon
        } else if message.starts_with("You don't seem to be solving the right level") {
            SubmissionStatus::WrongLevel
        } else {
            SubmissionStatus::Incorrect
        };

        let mut wait_minutes = 0;
        if submission_status == SubmissionStatus::Incorrect
            || submission_status == SubmissionStatus::TooSoon
        {
            wait_minutes = Self::extract_wait_time_from_message(&message);
        }

        Ok(SubmissionResult::new(
            submission,
            submission_status,
            message,
            chrono::Utc::now(),
            wait_minutes,
        ))
    }

    fn prepare_http_client(configuration: &Configuration) -> Client {
        let cookie = format!("session={}", configuration.aoc.token);
        let url = AOC_URL.parse::<Url>().expect("Invalid URL");
        let jar = Jar::default();
        jar.add_cookie_str(&cookie, &url);

        Client::builder()
            .cookie_provider(Arc::new(jar))
            .user_agent(Self::aoc_elf_user_agent())
            .build()
            .unwrap()
    }

    fn aoc_elf_user_agent() -> String {
        let pkg_name: &str = env!("CARGO_PKG_NAME");
        let pkg_version: &str = env!("CARGO_PKG_VERSION");

        format!("{}/{}", pkg_name, pkg_version)
    }

    fn get_aoc_answer_selector() -> Selector {
        Selector::parse("main > article > p").unwrap()
    }

    fn extract_wait_time_from_message(message: &str) -> i64 {
        let please_wait_position = match message.find("lease wait ") {
            Some(position) => position,
            None => return 0,
        };
        let minutes_position = please_wait_position + 11;
        let next_space_position = message[minutes_position..].find(' ').unwrap();
        let minutes = &message[minutes_position..minutes_position + next_space_position];
        if minutes == "one" {
            1
        } else {
            minutes.parse::<i64>().unwrap_or(0)
        }
    }

    fn parse_submission_answer_body(body: &str) -> Result<String> {
        let document: Html = Html::parse_document(body);
        let answer = document
            .select(&Self::get_aoc_answer_selector())
            .next()
            .unwrap();
        let answer_text = answer
            .text()
            .collect::<Vec<_>>()
            .iter()
            .map(|&s| s.trim())
            .collect::<Vec<_>>()
            .join(" ");

        Ok(answer_text)
    }

    fn failed_input_request_response() -> InputResponse {
        InputResponse::new("Failed to get input".to_string(), ResponseStatus::Error)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extraction_of_wait_time_from_message1() {
        let message = "That's not the right answer; your answer is too low. Please wait one minute and try again (you guessed 1).";
        let wait_time = AocApi::extract_wait_time_from_message(message);
        assert_eq!(wait_time, 1);
    }

    #[test]
    fn extraction_of_wait_time_from_message2() {
        let message = "That's not the right answer; your answer is too low. Please wait 2 minutes and try again (you guessed 1).";
        let wait_time = AocApi::extract_wait_time_from_message(message);
        assert_eq!(wait_time, 2);
    }
}
