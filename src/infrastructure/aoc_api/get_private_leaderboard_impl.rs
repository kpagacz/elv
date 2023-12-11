use anyhow::Context;

use crate::domain::{
    ports::{errors::AocClientError, get_private_leaderboard::GetPrivateLeaderboard},
    private_leaderboard::PrivateLeaderboard,
};
use crate::infrastructure::private_leaderboard::private_leaderboard_response::PrivateLeaderboardResponse;

use super::{AocApi, AOC_URL};

impl GetPrivateLeaderboard for AocApi {
    fn get_private_leaderboard(
        &self,
        leaderboard_id: &str,
        year: i32,
    ) -> Result<PrivateLeaderboard, AocClientError> {
        let url = reqwest::Url::parse(&format!(
            "{}/{}/leaderboard/private/view/{}.json",
            AOC_URL, year, leaderboard_id
        ))?;
        let response = self.http_client.get(url).send()?.error_for_status()?;
        PrivateLeaderboard::from_json(response)
    }
}

impl PrivateLeaderboard {
    pub fn from_json<T: std::io::Read>(mut readable: T) -> Result<Self, AocClientError> {
        let mut body = String::new();
        readable
            .read_to_string(&mut body)
            .map_err(|_| AocClientError::GetLeaderboardError)?;
        let response: PrivateLeaderboardResponse = serde_json::from_str(&body)
            .with_context(|| format!("Cannot parse the server response as a JSON that maps to PrivateLeaderboardResponse. Got response:\n{}...", &body[..200]))?;
        Ok(response.into())
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader, path::PathBuf};

    use super::*;
    #[test]
    fn test_from_json() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests/resources/private-leaderboard.json");
        let f = File::open(d).expect("Cannot open test file");
        let buf_reader = BufReader::new(f);
        let leaderboard = PrivateLeaderboard::from_json(buf_reader);
        assert!(leaderboard.is_ok());

        let leaderboard = leaderboard.unwrap();
        assert_eq!(leaderboard.entries.len(), 4);

        assert!(leaderboard.entries.first().unwrap().user == *"Konrad Pagacz");
        assert!(leaderboard.entries.last().unwrap().user == *"Anna Pytel");
        assert!(leaderboard.entries.first().unwrap().points == 187)
    }
}
