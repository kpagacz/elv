use crate::{
    domain::{errors::*, Description},
    Configuration,
};

use super::CliDisplay;

pub struct HttpDescription {
    year: u16,
    day: u8,
    body: String,
}
impl HttpDescription {
    fn part_one(&self) -> Option<String> {
        let part_one_selector = scraper::Selector::parse(".day-desc").unwrap();
        let binding = scraper::Html::parse_document(&self.body);
        let select = binding.select(&part_one_selector);
        select.map(|e| e.inner_html()).next()
    }

    fn part_one_answer(&self) -> Option<String> {
        let part_one_answer_selector = scraper::Selector::parse("main > p").unwrap();
        let binding = scraper::Html::parse_document(&self.body);
        binding
            .select(&part_one_answer_selector)
            .map(|e| e.inner_html())
            .find(|html| html.starts_with("Your puzzle answer was"))
    }

    fn part_two(&self) -> Option<String> {
        let part_one_selector = scraper::Selector::parse(".day-desc").unwrap();
        let binding = scraper::Html::parse_document(&self.body);
        let select = binding.select(&part_one_selector);
        select.map(|e| e.inner_html()).skip(1).next()
    }

    fn part_two_answer(&self) -> Option<String> {
        let part_one_answer_selector = scraper::Selector::parse("main > p").unwrap();
        let binding = scraper::Html::parse_document(&self.body);
        binding
            .select(&part_one_answer_selector)
            .map(|e| e.inner_html())
            .filter(|html| html.starts_with("Your puzzle answer was"))
            .skip(1)
            .next()
    }
}

impl TryFrom<reqwest::blocking::Response> for HttpDescription {
    type Error = Error;

    fn try_from(http_response: reqwest::blocking::Response) -> Result<HttpDescription> {
        if http_response.status().is_success() == false {
            return Err("AoC server responded with an error".into());
        }

        let mut year = String::new();
        let mut day = String::new();
        let year_and_day_regex =
            regex::Regex::new(r".+\.com/([[:alnum:]]+)/day/([[:alnum:]]+)$").unwrap();
        match year_and_day_regex.captures(http_response.url().as_str()) {
            Some(captures) => {
                captures.expand("1", &mut year);
                captures.expand("2", &mut day);
            }
            None => error_chain::bail!(
                "Cannot extract year and day from the url to construct a Description"
            ),
        }

        Ok(HttpDescription {
            year: year.parse().chain_err(|| "Failed parsing the year")?,
            day: day.parse().chain_err(|| "Failed parsing the day")?,
            body: http_response
                .text()
                .chain_err(|| "Failed unwrapping the body of the response")?,
        })
    }
}

impl Description for HttpDescription {
    fn year(&self) -> u16 {
        self.year
    }

    fn day(&self) -> u8 {
        self.day
    }
}

impl std::fmt::Display for HttpDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl CliDisplay for HttpDescription {
    fn cli_fmt(&self, configuration: &Configuration) -> String {
        let description = [
            self.part_one(),
            self.part_one_answer(),
            self.part_two(),
            self.part_two_answer(),
        ]
        .iter()
        .filter(|part| part.is_some())
        .map(|part| part.as_deref().unwrap())
        .collect::<Vec<_>>()
        .join("\n");
        html2text::from_read_with_decorator(
            description.as_bytes(),
            configuration.cli.output_width,
            html2text::render::text_renderer::TrivialDecorator::new(),
        )
    }
}
