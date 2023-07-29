#[derive(thiserror::Error, Debug, PartialEq)]
pub enum RiddleDateError {
    #[error("Could not guess the riddle date based on the current date")]
    GuessError,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RiddleDate {
    pub year: i32,
    pub day: i32,
}

impl RiddleDate {
    pub fn new(year: i32, day: i32) -> Self {
        RiddleDate { year, day }
    }

    pub fn best_guess<Date: chrono::Datelike>(
        year: Option<i32>,
        day: Option<i32>,
        current_date: Date,
    ) -> Result<Self, RiddleDateError> {
        match (year, day) {
            (Some(year), Some(day)) => Ok(Self::new(year, day)),
            (None, None) => Self::guess_from_current_date(current_date),
            (None, Some(day)) => Self::guess_from_day(day, current_date),
            (Some(_), None) => Err(RiddleDateError::GuessError),
        }
    }

    fn guess_from_current_date<Date: chrono::Datelike>(
        current_date: Date,
    ) -> Result<Self, RiddleDateError> {
        if current_date.month() == 12 && current_date.day() <= 25 {
            Ok(Self::new(current_date.year(), current_date.day() as i32))
        } else {
            Err(RiddleDateError::GuessError)
        }
    }

    fn guess_from_day<Date: chrono::Datelike>(
        day: i32,
        current_date: Date,
    ) -> Result<Self, RiddleDateError> {
        if current_date.month() == 12 {
            Ok(Self::new(current_date.year(), day))
        } else {
            Ok(Self::new(current_date.year() - 1, day))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guess_from_the_exact_year_and_day() {
        let expected = RiddleDate::new(1, 1);
        assert_eq!(
            Ok(expected),
            RiddleDate::best_guess(Some(1), Some(1), chrono::Utc::now())
        );
    }

    #[test]
    fn guess_current_date_if_its_december() {
        let now = chrono::NaiveDate::from_ymd_opt(2023, 12, 3).unwrap();
        let expected = RiddleDate::new(2023, 3);
        assert_eq!(Ok(expected), RiddleDate::best_guess(None, None, now));
    }

    #[test]
    fn cannot_guess_from_current_date_if_its_not_december() {
        let now = chrono::NaiveDate::from_ymd_opt(2023, 11, 1).unwrap();
        assert!(RiddleDate::best_guess(None, None, now).is_err());
    }

    #[test]
    fn guess_last_year_if_the_year_is_not_given() {
        let now = chrono::NaiveDate::from_ymd_opt(2023, 7, 1).unwrap();
        let expected = RiddleDate::new(2022, 3);
        assert_eq!(Ok(expected), RiddleDate::best_guess(None, Some(3), now));
    }

    #[test]
    fn cannot_guess_if_only_the_year_is_provided() {
        assert!(RiddleDate::best_guess(Some(2023), None, chrono::Utc::now()).is_err());
    }
}
