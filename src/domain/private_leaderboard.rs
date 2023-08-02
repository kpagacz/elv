use super::solved_parts::SolvedParts;

#[derive(Debug, Clone)]
pub struct PrivateLeaderboard {
    pub entries: Vec<PrivateLeaderboardEntry>,
}

#[derive(Debug, Clone)]
pub struct PrivateLeaderboardEntry {
    pub user: String,
    pub points: i32,
    pub stars: Vec<SolvedParts>,
}
