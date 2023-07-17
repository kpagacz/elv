use crate::domain::errors::*;
use crate::Configuration;

use super::{AocApi, AOC_URL};

impl AocApi {
    pub fn new(http_client: reqwest::blocking::Client, configuration: Configuration) -> AocApi {
        Self {
            http_client,
            configuration,
        }
    }

    pub fn prepare_http_client(configuration: &Configuration) -> reqwest::blocking::Client {
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

    pub fn aoc_elf_user_agent() -> String {
        let pkg_name: &str = env!("CARGO_PKG_NAME");
        let pkg_version: &str = env!("CARGO_PKG_VERSION");

        format!(
            "{}/{} (+{} author:{})",
            pkg_name, pkg_version, "https://github.com/kpagacz/elv", "konrad.pagacz@gmail.com"
        )
    }

    pub fn extract_wait_time_from_message(message: &str) -> std::time::Duration {
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

    pub fn get_aoc_answer_selector() -> scraper::Selector {
        scraper::Selector::parse("main > article > p").unwrap()
    }

    pub fn parse_submission_answer_body(self: &Self, body: &str) -> Result<String> {
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
