use super::super::leaderboard::Leaderboard;

use super::errors::AocClientError;

pub trait GetLeaderboard {
    fn get_leaderboard(&self, year: i32) -> Result<Leaderboard, AocClientError>;
}
