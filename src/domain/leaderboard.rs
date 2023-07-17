use error_chain::bail;

use crate::domain::errors::*;

#[derive(PartialEq, Debug)]
pub struct LeaderboardEntry {
    pub position: i32,
    pub points: i32,
    pub username: String,
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
        entry_username = values
            .iter()
            .skip(2)
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        Ok(Self {
            position: entry_position.replace(r")", "").parse().chain_err(|| {
                format!("Error parsing a leaderboard position: {}", entry_position)
            })?,
            points: entry_points
                .parse()
                .chain_err(|| format!("Error parsing points: {}", entry_points))?,
            username: entry_username,
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct Leaderboard {
    pub entries: Vec<LeaderboardEntry>,
}

impl FromIterator<LeaderboardEntry> for Leaderboard {
    fn from_iter<T: IntoIterator<Item = LeaderboardEntry>>(iter: T) -> Self {
        Self {
            entries: iter.into_iter().collect(),
        }
    }
}

impl TryFrom<Vec<String>> for Leaderboard {
    type Error = Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let entries: Result<Vec<LeaderboardEntry>> = value
            .iter()
            .map(|entry| LeaderboardEntry::try_from(entry.as_ref()))
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
        let expected_entry = {
            let username = "betaveros".to_owned();
            LeaderboardEntry {
                position: 1,
                points: 3693,
                username,
            }
        };
        let result_entry = LeaderboardEntry::try_from(entry);
        match result_entry {
            Ok(result) => assert_eq!(expected_entry, result),
            Err(e) => panic!("error parsing the entry: {}", e.description()),
        }
    }

    #[test]
    fn try_from_for_leaderboard_entry_anonymous_user() {
        let entry = "3) 3042  (anonymous user #1510407)";
        let expected_entry = {
            let username = "(anonymous user #1510407)".to_owned();
            LeaderboardEntry {
                position: 3,
                points: 3042,
                username,
            }
        };
        let result_entry = LeaderboardEntry::try_from(entry);
        match result_entry {
            Ok(result) => assert_eq!(expected_entry, result),
            Err(e) => panic!("error parsing the entry: {}", e.description()),
        }
    }

    #[test]
    fn try_from_string_vec_for_leaderboard() {
        let entries: Vec<String> = vec!["1) 3693  betaveros", "2) 14 me"]
            .iter()
            .map(|&x| x.to_owned())
            .collect();
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
