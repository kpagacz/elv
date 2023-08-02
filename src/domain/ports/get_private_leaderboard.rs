use crate::domain::private_leaderboard::PrivateLeaderboard;

use super::errors::AocClientError;

pub trait GetPrivateLeaderboard {
    fn get_private_leaderboard(
        &self,
        leaderboard_id: &str,
        year: i32,
    ) -> Result<PrivateLeaderboard, AocClientError>;
}
