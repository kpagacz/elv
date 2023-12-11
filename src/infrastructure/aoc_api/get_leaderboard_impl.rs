use std::io::Read;

use crate::domain::{
    leaderboard::{Leaderboard, LeaderboardError},
    ports::{errors::AocClientError, get_leaderboard::GetLeaderboard},
};

use super::{AocApi, AOC_URL};

impl GetLeaderboard for AocApi {
    fn get_leaderboard(&self, year: i32) -> Result<Leaderboard, AocClientError> {
        let url = reqwest::Url::parse(&format!("{}/{}/leaderboard", AOC_URL, year))?;
        let mut response = self.http_client.get(url).send()?.error_for_status()?;
        let mut body = String::from("");
        response.read_to_string(&mut body)?;

        Ok(Self::parse_leaderboard_response(body)?)
    }
}

impl AocApi {
    fn parse_leaderboard_response(response_body: String) -> Result<Leaderboard, LeaderboardError> {
        let leaderboard_entries_selector =
            scraper::Selector::parse(".leaderboard-entry").expect("Error parsing the css selector");
        let html = scraper::Html::parse_document(&response_body);
        let leaderboard_position_selector =
            scraper::Selector::parse(".leaderboard-position").unwrap();
        let leaderboard_points_selector =
            scraper::Selector::parse(".leaderboard-totalscore").unwrap();
        let entries = html
            .select(&leaderboard_entries_selector)
            .enumerate()
            .map(|(id, selected)| {
                let mut position = selected
                    .select(&leaderboard_position_selector)
                    .flat_map(|position| position.text())
                    .collect::<Vec<_>>()
                    .join("")
                    .trim()
                    .trim_end_matches(')')
                    .to_owned();
                if position.is_empty() {
                    position = id.to_string();
                }
                let points = selected
                    .select(&leaderboard_points_selector)
                    .flat_map(|points| points.text())
                    .collect::<Vec<_>>()
                    .join("")
                    .trim()
                    .to_owned();
                let mut name = selected
                    .children()
                    .filter_map(|node| match node.value() {
                        scraper::Node::Text(text) => Some(&text[..]),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("")
                    .trim()
                    .to_owned();
                if name.is_empty() {
                    let a_selector = scraper::Selector::parse(".leaderboard-anon, a").unwrap();
                    name = selected
                        .select(&a_selector)
                        .take(1)
                        .flat_map(|node| node.text())
                        .collect::<Vec<_>>()
                        .join("");
                }
                format!("{position} {points} {name}")
            })
            .collect::<Vec<_>>();

        Leaderboard::try_from(entries)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    #[test]
    fn parse_leaderboard_response() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests/resources/leaderboards.html");
        let contents = std::fs::read_to_string(d.into_os_string().into_string().unwrap()).unwrap();
        let leaderboard = AocApi::parse_leaderboard_response(contents);
        assert!(leaderboard.is_ok());
    }
}
