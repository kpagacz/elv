use crate::aoc_domain::{RiddlePart, Submission, SubmissionResult, SubmissionStatus};
use crate::configuration::Configuration;
use crate::errors::*;
use error_chain::bail;
use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, ORIGIN};
use reqwest::StatusCode;
use reqwest::{cookie::Jar, Url};
use scraper::{Html, Selector};
use std::io::Read;
use std::sync::Arc;

const AOC_URL: &str = "https://adventofcode.com";
const TERM_WIDTH: usize = 120;

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
            Err(_) => return InputResponse::failed(),
        };
        if response.status() != StatusCode::OK {
            return InputResponse::failed();
        }
        let mut body = String::new();
        if response.read_to_string(&mut body).is_err() {
            return InputResponse::failed();
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
        if !response.status().is_success() {
            bail!(
                "Failed to submit the answer. Status code: {}",
                response.status()
            );
        }

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

    /// Queries the Advent of Code website for the description of a riddle
    /// for a given day and year and returns it as a formatted string.
    pub fn get_description(&self, year: &u16, day: &u8) -> Result<String> {
        let url = Url::parse(&format!("{}/{}/day/{}", AOC_URL, year, day))
            .chain_err(|| "Failed to form the url for the description")?;
        let mut response = self
            .http_client
            .get(url)
            .send()
            .chain_err(|| "Failed to send the request for the description")?;

        if !response.status().is_success() {
            bail!(
                "Failed to get the description. Status code: {}",
                response.status()
            );
        }

        let mut body = String::new();
        response
            .read_to_string(&mut body)
            .chain_err(|| "Failed to read the response body")?;

        let description_selector = Selector::parse(".day-desc").unwrap();
        let description = Html::parse_document(&body)
            .select(&description_selector)
            .map(|e| e.inner_html())
            .collect::<Vec<_>>()
            .join("\n");
        Ok(html2text::from_read_with_decorator(
            description.as_bytes(),
            TERM_WIDTH,
            html2text::render::text_renderer::TrivialDecorator::new(),
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
            .chain_err(|| "Failed to create HTTP client")
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
            .chain_err(|| "Failed to parse the answer")?;
        let answer_text = html2text::from_read(
            answer.text().collect::<Vec<_>>().join("").as_bytes(),
            TERM_WIDTH,
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
        assert_eq!(wait_time, 1);
    }

    #[test]
    fn extraction_of_wait_time_from_message2() {
        let message = "That's not the right answer; your answer is too low. Please wait 2 minutes and try again (you guessed 1).";
        let wait_time = AocApi::extract_wait_time_from_message(message);
        assert_eq!(wait_time, 2);
    }

    #[test]
    fn test_parse_submission_answer_body() {
        let body = r#"
        <main>
<article><p>That's the right answer!  You
 are <span class="day-success">one gold star</span> closer to saving your
 vacation. <a href="/2020/day/1#part2">[Continue to Part Two]</a></p></article>
</main>"#;

        let message = AocApi::parse_submission_answer_body(body).unwrap();
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

        let message = AocApi::parse_submission_answer_body(body).unwrap();
        assert_eq!(message, concat!(
            "That's not the right answer. If you're stuck, make sure you're using the full input data; there are also some general\n",
            "tips on the about page, or you can ask for hints on the subreddit. ",
            "Because you have guessed incorrectly 7 times on this\npuzzle, please ",
            "wait 10 minutes before trying again. (You guessed 0.) [Return to Day 1]\n"))
    }
}
