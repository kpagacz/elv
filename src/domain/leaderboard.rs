use error_chain::bail;

use crate::domain::errors::*;

#[derive(PartialEq, Debug)]
struct LeaderboardEntry {
    position: i32,
    points: i32,
    username: String,
}

impl LeaderboardEntry {
    fn new(position: i32, points: i32, username: String) -> Self {
        LeaderboardEntry {
            position,
            points,
            username,
        }
    }
}

impl TryFrom<&str> for LeaderboardEntry {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let values: Vec<&str> = value.split_whitespace().collect();
        let (entry_position, entry_points, entry_username);
        if let Some(&position) = values.get(0) {
            entry_position = position;
        } else {
            bail!("No leaderboard position");
        }
        if let Some(&points) = values.get(1) {
            entry_points = points;
        } else {
            bail!("No points in a leaderboard entry");
        }
        if let Some(&username) = values.get(2) {
            entry_username = username;
        } else {
            bail!("No username in a leaderboard entry");
        }

        Ok(Self {
            position: entry_position
                .replace(r")", "")
                .parse()
                .chain_err(|| "Error parsing a leaderboard position")?,
            points: entry_points
                .parse()
                .chain_err(|| "Error parsing points in a leaderboard position")?,
            username: entry_username.to_owned(),
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct Leaderboard {
    entries: Vec<LeaderboardEntry>,
}

impl FromIterator<LeaderboardEntry> for Leaderboard {
    fn from_iter<T: IntoIterator<Item = LeaderboardEntry>>(iter: T) -> Self {
        Self {
            entries: iter.into_iter().collect(),
        }
    }
}

impl TryFrom<Vec<&str>> for Leaderboard {
    type Error = Error;

    fn try_from(value: Vec<&str>) -> Result<Self> {
        let entries: Result<Vec<LeaderboardEntry>> = value
            .iter()
            .map(|&entry| LeaderboardEntry::try_from(entry))
            .collect();
        match entries {
            Ok(entries) => Ok(Leaderboard::from_iter(entries)),
            Err(e) => bail!(e.chain_err(|| "One of the entries failed parsing")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_from_for_leaderboard_entry() {
        let entry = "1) 3693  betaveros";
        let expected_entry = LeaderboardEntry::new(1, 3693, "betaveros".to_owned());
        let result_entry = LeaderboardEntry::try_from(entry);
        match result_entry {
            Ok(result) => assert_eq!(expected_entry, result),
            Err(e) => panic!("error parsing the entry: {}", e.description()),
        }
    }

    #[test]
    fn try_from_for_leaderboard() {
        let entries = vec!["1) 3693  betaveros", "2) 14 me"];
        let expected_leaderboard = Leaderboard {
            entries: vec![
                LeaderboardEntry {
                    position: 1,
                    points: 3693,
                    username: "betaveros".to_owned(),
                },
                LeaderboardEntry {
                    position: 2,
                    points: 14,
                    username: "me".to_owned(),
                },
            ],
        };
        match Leaderboard::try_from(entries) {
            Ok(result) => assert_eq!(expected_leaderboard, result),
            Err(e) => panic!("Test case failed {}", e.description()),
        }
    }
}
