#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, clap::ValueEnum)]
pub enum RiddlePart {
    One = 1,
    Two = 2,
}

impl std::fmt::Display for RiddlePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiddlePart::One => write!(f, "one"),
            RiddlePart::Two => write!(f, "two"),
        }
    }
}
