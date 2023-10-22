use super::solved_parts::SolvedParts;

#[derive(Debug, Clone)]
pub struct PrivateLeaderboard {
    pub entries: Vec<PrivateLeaderboardEntry>,
}

impl PrivateLeaderboard {
    pub fn new(mut entries: Vec<PrivateLeaderboardEntry>) -> Self {
        entries.sort_by(|first, second| second.points.cmp(&first.points));
        Self { entries }
    }
}

#[derive(Debug, Clone)]
pub struct PrivateLeaderboardEntry {
    pub user: String,
    pub points: usize,
    pub stars: Vec<SolvedParts>,
}
