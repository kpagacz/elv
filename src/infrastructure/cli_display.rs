use colored::Colorize;

use crate::{
    domain::{
        leaderboard::Leaderboard, private_leaderboard::PrivateLeaderboard,
        solved_parts::SolvedParts,
    },
    Configuration,
};

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

impl CliDisplay for PrivateLeaderboard {
    fn cli_fmt(&self, _configuration: &Configuration) -> String {
        fn stars(solved_status: &[SolvedParts]) -> String {
            solved_status
                .iter()
                .map(|status| match status {
                    SolvedParts::None => "*".dimmed(),
                    SolvedParts::One => "*".white(),
                    SolvedParts::Both => "*".yellow(),
                })
                .fold("".to_owned(), |acc, coloured_string| {
                    format!("{}{}", acc, coloured_string)
                })
        }
        let out = self
            .entries
            .iter()
            .enumerate()
            .map(|(rank, entry)| {
                format!(
                    "{}){:>4} {}  {}",
                    rank + 1,
                    entry.points,
                    stars(&entry.stars),
                    entry.user
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        const FIRST_LINE: &str = "                1111111111222222222";
        const SECOND_LINE: &str = "       12345678901234567890123456789";
        format!(
            "{}\n{}\n{}",
            &FIRST_LINE[..(7 + self.entries[0].stars.len())],
            &SECOND_LINE[..(7 + self.entries[0].stars.len())],
            out
        )
    }
}

#[cfg(test)]
mod tests {}
