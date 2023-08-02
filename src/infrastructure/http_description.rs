use crate::{domain::description::Description, Configuration};

use super::cli_display::CliDisplay;

pub struct HttpDescription {
    year: u16,
    day: u8,
    body: String,
}

impl HttpDescription {
    pub fn part_one(&self) -> Option<String> {
        let part_one_selector = scraper::Selector::parse(".day-desc").unwrap();
        let binding = scraper::Html::parse_document(&self.body);
        let select = binding.select(&part_one_selector);
        select.map(|e| e.inner_html()).next()
    }

    pub fn part_one_answer(&self) -> Option<String> {
        let part_one_answer_selector = scraper::Selector::parse("main > p").unwrap();
        let binding = scraper::Html::parse_document(&self.body);
        binding
            .select(&part_one_answer_selector)
            .map(|e| e.inner_html())
            .find(|html| html.starts_with("Your puzzle answer was"))
    }

    pub fn part_two(&self) -> Option<String> {
        let part_one_selector = scraper::Selector::parse(".day-desc").unwrap();
        let binding = scraper::Html::parse_document(&self.body);
        let select = binding.select(&part_one_selector);
        select.map(|e| e.inner_html()).skip(1).next()
    }

    pub fn part_two_answer(&self) -> Option<String> {
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
    type Error = anyhow::Error;

    fn try_from(
        http_response: reqwest::blocking::Response,
    ) -> Result<HttpDescription, anyhow::Error> {
        if http_response.status().is_success() == false {
            anyhow::bail!("AoC server responded with an error".to_owned());
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
            None => {
                anyhow::bail!("Cannot extract year and day from the url to construct a Description")
            }
        }

        Ok(HttpDescription {
            year: year.parse()?,
            day: day.parse()?,
            body: http_response.text()?,
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

        f.write_str(&html2text::from_read_with_decorator(
            description.as_bytes(),
            200,
            html2text::render::text_renderer::TrivialDecorator::new(),
        ))
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
            configuration.cli.output_width as usize,
            html2text::render::text_renderer::TrivialDecorator::new(),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn get_resource_file(file: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push(format!("tests/resources/{}", file));

        std::fs::read_to_string(d.as_path()).unwrap()
    }

    #[test]
    fn no_part_done() {
        let description = HttpDescription {
            year: 2022,
            day: 1,
            body: get_resource_file("riddle-description-no-part-done.html"),
        };

        assert!(description.part_one().is_some());
        assert!(description.part_two().is_none());
        assert!(description.part_one_answer().is_none());
        assert!(description.part_two_answer().is_none());
    }

    #[test]
    fn first_part_done() {
        let description = HttpDescription {
            year: 2022,
            day: 1,
            body: get_resource_file("riddle-description-first-part-done.html"),
        };

        assert!(description.part_one().is_some());
        assert!(description.part_one_answer().is_some());
        assert!(description.part_two().is_some());
        assert!(description.part_two_answer().is_none());
    }

    #[test]
    fn second_part_done() {
        let description = HttpDescription {
            year: 2022,
            day: 1,
            body: get_resource_file("riddle-description-both-parts-done.html"),
        };

        assert!(description.part_one().is_some());
        assert!(description.part_one_answer().is_some());
        assert!(description.part_two().is_some());
        assert!(description.part_two_answer().is_some());
    }
}
