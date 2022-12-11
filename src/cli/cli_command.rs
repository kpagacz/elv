#[derive(Debug, clap::Subcommand)]
pub enum CliCommand {
    /// Get the input for the challenge
    ///
    /// This command will download the input for the challenge and print it to
    /// the console (stdout). If the input has already been downloaded, it will
    /// be printed from the cache. If the input has not been downloaded, it will
    /// be downloaded and then printed. The input will be cached in the
    /// application's cache directory.
    Input,

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
}
