use super::commands::Commands;

#[derive(Debug, clap::Parser)]
#[command(name = "elf", about = "Advent of Code CLI", long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}
