use crate::aoc_domain::{RiddlePart, Submission, SubmissionResult, SubmissionStatus};
use crate::configuration::Configuration;
use crate::errors::*;
use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, ORIGIN};
use reqwest::{cookie::Jar, Url};
use scraper::{Html, Selector};
use std::io::Read;
use std::sync::Arc;

pub fn get_input(day: &u8, http_client: &Client) -> Result<String> {
    let url = Url::parse(&format!("https://adventofcode.com/2022/day/{}/input", day))?;
    let mut res = http_client.get(url).send()?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    Ok(body)
}

pub fn submit_answer(submission: Submission, http_client: &Client) -> Result<SubmissionResult> {
    let url = Url::parse(&format!(
        "https://adventofcode.com/{}/day/{}/answer",
        submission.year, submission.day
    ))?;
    let mut response = http_client
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

    let message = parse_submission_answer_body(&body)?;
    let submission_status = if message.starts_with("That's the right answer!") {
        SubmissionStatus::Correct
    } else {
        SubmissionStatus::Incorrect
    };

    let mut wait_minutes = 0;
    if submission_status == SubmissionStatus::Incorrect {
        wait_minutes = extract_wait_time_from_message(&message);
    }

    Ok(SubmissionResult::new(
        submission,
        submission_status,
        message,
        chrono::Utc::now(),
        wait_minutes,
    ))
}

pub fn prepare_http_client(configuration: &Configuration) -> Client {
    let cookie = format!("session={}", configuration.aoc.token);
    let url = "https://adventofcode.com/"
        .parse::<Url>()
        .expect("Invalid URL");
    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);

    Client::builder()
        .cookie_provider(Arc::new(jar))
        .user_agent(aoc_elf_user_agent())
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
    let please_wait_position = message.find("lease wait ");
    if please_wait_position == None {
        0
    } else {
        let minutes_position = please_wait_position.unwrap() + 11;
        let next_space_position = message[minutes_position..].find(" ").unwrap();
        let minutes = &message[minutes_position..minutes_position + next_space_position];
        if minutes == "one" {
            1
        } else {
            minutes.parse::<i64>().unwrap_or(0)
        }
    }
}

fn parse_submission_answer_body(body: &str) -> Result<String> {
    let document: Html = Html::parse_document(body);
    let answer = document.select(&get_aoc_answer_selector()).next().unwrap();
    let answer_text = answer
        .text()
        .collect::<Vec<_>>()
        .iter()
        .map(|&s| s.trim())
        .collect::<Vec<_>>()
        .join(" ");

    Ok(answer_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extraction_of_wait_time_from_message1() {
        let message = "That's not the right answer; your answer is too low. Please wait one minute and try again (you guessed 1).";
        let wait_time = extract_wait_time_from_message(message);
        assert_eq!(wait_time, 1);
    }

    #[test]
    fn extraction_of_wait_time_from_message2() {
        let message = "That's not the right answer; your answer is too low. Please wait 2 minutes and try again (you guessed 1).";
        let wait_time = extract_wait_time_from_message(message);
        assert_eq!(wait_time, 2);
    }
}
