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
            if submission.part == RiddlePart::One {
                1
            } else {
                2
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
    // *TODO* Determine other submission statuses (like Unknown and answer submitted too soon ather the previous one)

    Ok(SubmissionResult::new(
        submission,
        submission_status,
        message,
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

fn parse_submission_answer_body(body: &str) -> Result<String> {
    let document: Html = Html::parse_document(body);
    let answer = document
        .select(&Selector::parse("main > article > p").unwrap())
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
