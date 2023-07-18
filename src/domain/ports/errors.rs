use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocClientError {
    #[error("Failed to get the leaderboard")]
    GetLeaderboardError,

    #[error("Failed to get the description")]
    GetDescriptionError,

    #[error("Failed to get input")]
    GetInputError,

    #[error("Failed to submit the answer")]
    SubmitAnswerError,
}

#[derive(Error, Debug)]
pub enum InputCacheError {
    #[error("Failed saving to cache")]
    Save,

    #[error("Failed to load from cache")]
    Load,

    #[error("Failed to clear the cache")]
    Clear,
}
