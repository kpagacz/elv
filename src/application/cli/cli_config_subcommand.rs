#[derive(clap::Parser, Debug)]
pub enum ConfigSubcommand {
    /// Lists all the configuration keys and their respective values
    #[command(visible_aliases = ["l"])]
    List {},

    /// Updates the value of the specified configuration key
    ///
    /// Examples:
    /// elv config set aoc.token abscdft123145
    /// elv config set cli.output_width 150
    #[command(verbatim_doc_comment)]
    Set {
        /// The updated configuration key
        key: String,

        /// The value used to update the key
        value: String,
    },
}
