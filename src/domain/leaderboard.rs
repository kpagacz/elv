use thiserror::Error;

#[derive(PartialEq, Debug)]
pub struct LeaderboardEntry {
    pub position: i32,
    pub points: i32,
    pub username: String,
}

#[derive(Error, Debug)]
pub enum LeaderboardEntryError {
    #[error("No position in the leaderboard entry")]
    Position,

    #[error("No points in the leaderboard entry")]
    Points,

    #[error("Error parsing: {}", 0)]
    Parsing(String),
}

impl TryFrom<&str> for LeaderboardEntry {
    type Error = LeaderboardEntryError;

    fn try_from(value: &str) -> Result<Self, LeaderboardEntryError> {
        let values: Vec<&str> = value.split_whitespace().collect();
        let (entry_position, entry_points, entry_username);
        entry_position = values.get(0).ok_or(LeaderboardEntryError::Position)?;
        entry_points = values.get(1).ok_or(LeaderboardEntryError::Points)?;
        entry_username = values
            .iter()
            .skip(2)
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        Ok(Self {
            position: entry_position
                .replace(r")", "")
                .parse()
                .or(LeaderboardEntryError::Parsing("position".to_owned()))?,
            points: entry_points
                .parse()
                .or(LeaderboardEntryError::Parsing("points".to_owned()))?,
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

#[derive(Error, Debug)]
#[error(transparent)]
pub struct LeaderboardError(#[from] LeaderboardEntryError);

impl TryFrom<Vec<String>> for Leaderboard {
    type Error = LeaderboardError;

    fn try_from(value: Vec<String>) -> Result<Self, LeaderboardError> {
        let entries: Result<Vec<LeaderboardEntry>> = value
            .iter()
            .map(|entry| LeaderboardEntry::try_from(entry.as_ref()))
            .collect();
        Ok(Leaderboard::from_iter(entries?))
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
