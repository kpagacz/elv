use crate::domain::Leaderboard;

use super::errors::AocClientError;

pub trait GetLeaderboard {
    fn get_leaderboard(&self, year: u16) -> Result<Leaderboard, AocClientError>;
}
