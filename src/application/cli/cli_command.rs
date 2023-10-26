use std::path::PathBuf;

use clap::Args;

use crate::domain::riddle_part::RiddlePart;

use super::cli_config_subcommand::ConfigSubcommand;

#[derive(Debug, Args)]
pub struct RiddleArgs {
    /// The year of the challenge
    ///
    /// If you do not supply a year and a day, the current year will be used.
    /// If you do not supply a year, but supply a day, the previous year
    /// will be used.
    #[arg(short, long, value_parser = clap::value_parser!(usize))]
    pub year: Option<usize>,

    /// The day of the challenge
    ///
    /// If you do not supply a day, the current day of the month will be used
    /// (if the current month is December). If the current month is not December,
    /// the application will not be able to guess the day.
    #[arg(short, long, value_parser = clap::value_parser!(usize))]
    pub day: Option<usize>,
}

#[derive(Debug, Args)]
pub struct TokenArgs {
    /// Your Advent of Code session token
    ///
    /// You can find your session token by logging into Advent of Code and
    /// inspecting the cookies of the page. The session token is the value of
    /// the cookie named "session".
    #[arg(short, long)]
    pub token: Option<String>,
}

#[derive(Debug, clap::Subcommand)]
pub enum CliCommand {
    /// 📄 Get the description of the challenge
    ///
    /// This command will download the description of the challenge and write it to
    /// the console.
    #[command(visible_aliases = ["desc", "d"])]
    Description {
        #[command(flatten)]
        riddle_args: RiddleArgs,

        #[command(flatten)]
        token: TokenArgs,

        /// The column width of the output in characters
        ///
        /// Some terminals have limited horizontal space, so this option
        /// can be used to limit the width of the output.
        #[arg(short, long, default_value_t = 120)]
        width: usize,
    },

    /// 📨 Get the input for the challenge
    ///
    /// This command will download the input for the challenge and write it to
    /// a file. The default file name is "input". If the input has already
    /// been downloaded, it will be printed from the cache. If the input
    /// has not been downloaded, it will be downloaded and then printed.
    /// The input will be cached in the application's cache directory.
    #[command(visible_aliases = ["i"])]
    Input {
        #[command(flatten)]
        riddle_args: RiddleArgs,

        #[command(flatten)]
        token_args: TokenArgs,

        /// The input will be written to the file with this name
        #[arg(short, long, default_value = "input", conflicts_with = "no_file")]
        out: PathBuf,

        /// Suppresses writing to the file
        #[arg(short, long, default_value = "false", conflicts_with = "out")]
        no_file: bool,

        /// Prints the input to stdout as well as writing it to a file
        #[arg(short, long, default_value = "false")]
        print: bool,
    },

    /// 🎯 Submit an answer to the challenge
    ///
    /// This command will submit an answer to the challenge. If the answer has
    /// already been submitted, the result of the submission will be printed.
    /// Otherwise, the answer will be submitted and the result will be printed.
    /// The result of the submission will be cached in the application's cache
    /// directory.
    #[command(visible_aliases = ["s"])]
    Submit {
        /// The answer to the challenge
        ///
        /// Your answer to the challenge. This argument is required.
        answer: String,

        /// The part of the challenge
        ///
        /// Possible values: "one", "two".
        part: Option<RiddlePart>,

        #[command(flatten)]
        riddle_args: RiddleArgs,

        #[command(flatten)]
        token_args: TokenArgs,
    },

    /// 🥇 Show the leaderboard
    ///
    /// This command downloads the leaderboard rankings for a particular year.
    #[command(visible_aliases = ["l"])]
    Leaderboard {
        #[command(flatten)]
        token_args: TokenArgs,

        /// The year of the challenge
        ///
        /// If you do not supply a year, this command will pull the leaderboards from
        /// the latest event.
        #[arg(short, long, value_parser = clap::value_parser!(i32))]
        year: Option<i32>,
    },

    /// 🥇 Show a private leaderboard
    ///
    /// This command downloads the leaderboard rankings for a particular year.
    #[command(visible_aliases = ["pl"])]
    PrivateLeaderboard {
        #[command(flatten)]
        token_args: TokenArgs,

        /// The ID of the leaderboard
        ///
        /// The ID of the leaderboard is the part of the last part of the URL of
        /// the leaderboard you want to visit.
        #[arg(short, long, visible_aliases = ["id"])]
        leaderboard_id: String,

        /// The year of the challenge
        ///
        /// If you do not supply a year, this command will pull the leaderboards from
        /// the latest event.
        #[arg(short, long, value_parser = clap::value_parser!(i32))]
        year: Option<i32>,
    },

    /// ⭐ Show the stars page
    ///
    /// This command downloads the star page and displays the ASCII pattern along with
    /// the stars.
    Stars {
        /// The year of the challenge
        ///
        /// If you do not supply a year, this command will pull the leaderboards from
        /// the latest event.
        year: Option<i32>,
    },

    /// 🗑️  Clear the cache
    ///
    /// This command will clear the cache of the application. The cache is used
    /// to store the input and the results of submissions. This command will
    /// delete the cache directories and all of their contents.
    ClearCache,

    /// 📁 List the application directories
    ///
    /// Lists the directories used by the application. This command will print
    /// the directories used for the cache and the configuration file.
    ListDirs,

    /// 🔍 Show and edit the configuration
    ///
    /// Token management
    /// You can save your Advent of Code token using this command.
    /// See `elv --help` and `elv config set --help` for more information.
    #[command(verbatim_doc_comment)]
    Config {
        #[clap(subcommand)]
        cmd: ConfigSubcommand,
    },

    /// 🪙 Set the Advent Of Code token
    ///
    /// Token management
    /// You can save your Advent of Code token using this command.
    /// If you don't give any parameter to this command, it will print
    /// the currently saved token instead.
    /// Example:
    /// > elv token my_token
    /// > elv token
    /// my_token
    #[command(verbatim_doc_comment, visible_aliases = ["t", "sett", "set-token"])]
    Token {
        /// Token to be saved
        token: Option<String>,
    },
}
