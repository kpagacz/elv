use super::cli_command::CliCommand;

/// 🎄 Your Advent of Code CLI 🎄
///
/// This CLI is a tool to help you with your Advent of Code challenges.
///
/// Token management
/// You need an Advent of Code session token to interact with its API. `elv`
/// does not support authentication to the API on its own, so you need to
/// get your token beforehand, and pass it to `elv`. There are a number of ways
/// of setting the token. See `elv config set --help` if you want to set it
/// once and not be bothered by passing it to the `--token` parameter every
/// time you use the CLI.
#[derive(Debug, clap::Parser)]
#[command(version, verbatim_doc_comment)]
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
