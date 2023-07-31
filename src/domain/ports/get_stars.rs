use super::{super::stars::Stars, errors::AocClientError};

pub trait GetStars {
    fn get_stars(&self, year: i32) -> Result<Stars, AocClientError>;
}
