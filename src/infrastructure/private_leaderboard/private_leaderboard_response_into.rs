use crate::domain::{
    private_leaderboard::{PrivateLeaderboard, PrivateLeaderboardEntry},
    solved_parts::SolvedParts,
};

use super::private_leaderboard_response::{MemberResults, PrivateLeaderboardResponse};

impl From<PrivateLeaderboardResponse> for PrivateLeaderboard {
    fn from(value: PrivateLeaderboardResponse) -> Self {
        let entries: Vec<PrivateLeaderboardEntry> = value
            .members
            .values()
            .map(|member_results| member_results.into())
            .collect();
        PrivateLeaderboard::new(entries)
    }
}

impl From<&MemberResults> for PrivateLeaderboardEntry {
    fn from(value: &MemberResults) -> Self {
        let user = value.name.to_owned();
        let points = value.local_score;
        let mut stars = vec![SolvedParts::None; 25];
        value
            .completion_day_level
            .iter()
            .for_each(|(&day, completion)| {
                stars[day - 1] = match (&completion.stage_one, &completion.stage_two) {
                    (Some(_), None) => SolvedParts::One,
                    (Some(_), Some(_)) => SolvedParts::Both,
                    (_, _) => SolvedParts::None,
                };
            });

        PrivateLeaderboardEntry {
            user,
            points,
            stars,
        }
    }
}
