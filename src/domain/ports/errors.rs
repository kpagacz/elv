use thiserror::Error;

use super::super::leaderboard::LeaderboardError;

#[derive(Error, Debug)]
pub enum AocClientError {
    #[error("Failed to get the leaderboard")]
    GetLeaderboardError,

    #[error("Failed to submit the answer: {}", 0)]
    SubmitAnswerError(String),

    #[error("IO error")]
    IoErrorr(#[from] std::io::Error),

    #[error("URL parsing")]
    URLParsingError(#[from] url::ParseError),

    #[error("Runtime error")]
    RuntimeError(#[from] anyhow::Error),

    #[error("Network error")]
    NetworkError(#[from] reqwest::Error),
}

impl From<LeaderboardError> for AocClientError {
    fn from(_cause: LeaderboardError) -> Self {
        Self::GetLeaderboardError
    }
}
