use super::solved_parts::SolvedParts;

pub struct Stars {
    pub stars: Vec<SolvedParts>,
    pub pattern: Vec<String>,
}

impl Stars {
    pub fn new(stars: Vec<SolvedParts>, pattern: Vec<String>) -> Self {
        Stars { stars, pattern }
    }
}

impl core::fmt::Display for Stars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.pattern.join("\n"))
    }
}
