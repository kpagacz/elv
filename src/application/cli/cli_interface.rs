use super::cli_command::CliCommand;

/// 🎄 Your Advent of Code CLI 🎄
///
/// This CLI is a tool to help you with your Advent of Code challenges.
#[derive(Debug, clap::Parser)]
#[command(version)]
pub struct CliInterface {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_parses() {
        use clap::CommandFactory;
        CliInterface::command().debug_assert()
    }
}
