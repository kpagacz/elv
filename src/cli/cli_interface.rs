use super::cli_command::CliCommand;

/// 🎄 Your Advent of Code CLI 🎄
///
/// This CLI is a tool to help you with your Advent of Code challenges.
#[derive(Debug, clap::Parser)]
#[command(version)]
pub struct CliInterface {
    /// Your Advent of Code session token
    ///
    /// You can find your session token by logging into Advent of Code and
    /// inspecting the cookies of the page. The session token is the value of
    /// the cookie named "session".
    #[arg(short, long)]
    pub token: Option<String>,

    /// The year of the challenge
    ///
    /// If you do not supply a year, the current year will be used.
    #[arg(short, long, requires("day"), value_parser = clap::value_parser!(u16))]
    pub year: Option<u16>,

    /// The day of the challenge
    ///
    /// If you do not supply a day, the current day of the month will be used
    /// (if the current month is December).
    #[arg(short, long, requires("year"), value_parser = clap::value_parser!(u8))]
    pub day: Option<u8>,

    #[command(subcommand)]
    pub command: CliCommand,
}
