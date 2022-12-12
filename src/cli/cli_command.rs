use std::path::{PathBuf};

#[derive(Debug, clap::Subcommand)]
pub enum CliCommand {
    /// Get the input for the challenge
    ///
    /// This command will download the input for the challenge and write it to
    /// a file. The default file name is "input". If the input has already
    /// been downloaded, it will be printed from the cache. If the input
    /// has not been downloaded, it will be downloaded and then printed.
    /// The input will be cached in the application's cache directory.
    Input {
        /// The input will be written to the file with this name
        #[arg(
            short,
            long,
            default_value = "input",
            conflicts_with = "no_file"
        )]
        out: PathBuf,

        /// Suppresses writing to the file
        #[arg(short, long, default_value = "false", conflicts_with = "out")]
        no_file: bool,

        /// Prints the input to stdout as well as writing it to a file
        #[arg(short, long, default_value = "false")]
        print: bool,
    },

    /// Submit an answer to the challenge
    ///
    /// This command will submit an answer to the challenge. If the answer has
    /// already been submitted, the result of the submission will be printed.
    /// Otherwise, the answer will be submitted and the result will be printed.
    /// The result of the submission will be cached in the application's cache
    /// directory.
    Submit {
        /// The part of the challenge
        ///
        /// This argument is required. It must be either "one" or "two".
        part: crate::aoc_domain::RiddlePart,

        /// The answer to the challenge
        ///
        /// Your answer to the challenge. This argument is required.
        answer: String,
    },

    /// Clears the cache
    ///
    /// This command will clear the cache of the application. The cache is used
    /// to store the input and the results of submissions. This command will
    /// delete the cache directories and all of their contents.
    ClearCache,
}
