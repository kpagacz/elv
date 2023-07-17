use crate::domain::errors::*;
use crate::domain::Leaderboard;

pub trait GetLeaderboard {
    fn get_leaderboard(&self, year: u16) -> Result<Leaderboard>;
}
