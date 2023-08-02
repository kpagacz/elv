use crate::domain::{
    ports::{errors::AocClientError, get_private_leaderboard::GetPrivateLeaderboard},
    private_leaderboard::PrivateLeaderboard,
};

use super::{AocApi, AOC_URL};

impl GetPrivateLeaderboard for AocApi {
    fn get_private_leaderboard(
        &self,
        leaderboard_id: &str,
        year: i32,
    ) -> Result<PrivateLeaderboard, AocClientError> {
        let url = reqwest::Url::parse(&format!(
            "{}/{}/leaderboard/private/view/{}",
            AOC_URL, year, leaderboard_id
        ))?;
        let mut response = self.http_client.get(url).send()?.error_for_status()?;
        PrivateLeaderboard::from_readable(response)
    }
}

impl PrivateLeaderboard {
    pub fn from_readable<T: std::io::Read>(mut readable: T) -> Result<Self, AocClientError> {
        let mut body = String::new();
        readable
            .read_to_string(&mut body)
            .map_err(|_| AocClientError::GetLeaderboardError)?;

        parse_http_response(&body)
    }
}

fn parse_http_response(body: &str) -> Result<PrivateLeaderboard, AocClientError> {
    let document = scraper::Html::parse_document(&body);
}
