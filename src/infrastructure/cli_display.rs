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
                    SolvedParts::None => "*".truecolor(0x69, 0x69, 0x69),
                    SolvedParts::One => "*".white(),
                    SolvedParts::Both => "*".yellow(),
                })
                .fold("".to_owned(), |acc, coloured_string| {
                    format!("{}{}", acc, coloured_string)
                })
        }
        let out = std::iter::zip(1..=self.entries.len(), self.entries.iter())
            .map(|(rank, entry)| {
                format!(
                    "{}){:>4} {}  {}",
                    rank,
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
mod tests {
    use crate::domain::private_leaderboard::PrivateLeaderboardEntry;

    use super::*;

    #[test]
    fn for_private_leaderboard() {
        use SolvedParts;
        let leaderboard = PrivateLeaderboard {
            entries: vec![
                PrivateLeaderboardEntry {
                    user: "user 1".to_owned(),
                    points: 1,
                    stars: vec![
                        SolvedParts::One,
                        SolvedParts::Both,
                        SolvedParts::None,
                        SolvedParts::One,
                        SolvedParts::Both,
                        SolvedParts::None,
                        SolvedParts::One,
                        SolvedParts::Both,
                        SolvedParts::None,
                        SolvedParts::One,
                        SolvedParts::Both,
                        SolvedParts::None,
                    ],
                },
                PrivateLeaderboardEntry {
                    user: "user with a very long name".to_owned(),
                    points: 9999,
                    stars: vec![
                        SolvedParts::Both,
                        SolvedParts::Both,
                        SolvedParts::Both,
                        SolvedParts::Both,
                        SolvedParts::Both,
                        SolvedParts::Both,
                        SolvedParts::Both,
                        SolvedParts::Both,
                        SolvedParts::Both,
                        SolvedParts::Both,
                        SolvedParts::Both,
                        SolvedParts::Both,
                    ],
                },
            ],
        };

        assert_eq!("                111\n       123456789012\n1)   1 \u{1b}[37m*\u{1b}[0m\u{1b}[33m*\u{1b}[0m\u{1b}[38;2;105;105;105m*\u{1b}[0m\u{1b}[37m*\u{1b}[0m\u{1b}[33m*\u{1b}[0m\u{1b}[38;2;105;105;105m*\u{1b}[0m\u{1b}[37m*\u{1b}[0m\u{1b}[33m*\u{1b}[0m\u{1b}[38;2;105;105;105m*\u{1b}[0m\u{1b}[37m*\u{1b}[0m\u{1b}[33m*\u{1b}[0m\u{1b}[38;2;105;105;105m*\u{1b}[0m  user 1\n2)9999 \u{1b}[33m*\u{1b}[0m\u{1b}[33m*\u{1b}[0m\u{1b}[33m*\u{1b}[0m\u{1b}[33m*\u{1b}[0m\u{1b}[33m*\u{1b}[0m\u{1b}[33m*\u{1b}[0m\u{1b}[33m*\u{1b}[0m\u{1b}[33m*\u{1b}[0m\u{1b}[33m*\u{1b}[0m\u{1b}[33m*\u{1b}[0m\u{1b}[33m*\u{1b}[0m\u{1b}[33m*\u{1b}[0m  user with a very long name", leaderboard.cli_fmt(&Configuration::new()));
    }
}
