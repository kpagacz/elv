use super::super::stars::Stars;

pub trait GetStars {
    fn get_stars(&self, year: i32) -> anyhow::Result<Stars>;
}
