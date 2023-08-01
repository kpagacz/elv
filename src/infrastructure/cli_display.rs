use crate::{domain::Leaderboard, Configuration};

pub trait CliDisplay {
    fn cli_fmt(&self, configuration: &Configuration) -> String;
}

impl CliDisplay for Leaderboard {
    fn cli_fmt(&self, _configuration: &Configuration) -> String {
        self.entries
            .iter()
            .map(|entry| format!("{}) {} {}", entry.position, entry.points, entry.username))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
