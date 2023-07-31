use super::solved_parts::SolvedParts;

pub struct Stars {
    stars: Vec<SolvedParts>,
    pattern: Vec<String>,
}

impl Stars {
    pub fn new(stars: Vec<SolvedParts>, pattern: Vec<String>) -> Self {
        Stars { stars, pattern }
    }
}
