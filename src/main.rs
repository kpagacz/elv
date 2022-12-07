use aoc_elf::{Cli, CliCommand, Configuration, Driver};
use chrono::Datelike;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    let configuration: Configuration;
    if let Some(token) = cli.token {
        let builder = Configuration::builder()
            .set_override("aoc.token", token)
            .expect("Failed to set token");
        configuration = builder
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap_or_else(|_| {
                println!("Failed to deserialize the configuration, using default");
                Configuration::new()
            })
    } else {
        configuration = Configuration::new();
    }

    let mut day = cli.day;
    let mut year = cli.year;
    if day.is_none() || year.is_none() {
        let now = chrono::Utc::now() - chrono::Duration::hours(4);
        if day.is_none() {
            day = Some(now.day() as u8);
        }
        if year.is_none() {
            year = Some(now.year() as u16);
        }
    }

    let driver = Driver::new(configuration);
    match cli.command {
        CliCommand::Input => driver.input(year.unwrap(), day.unwrap()),
        CliCommand::Submit { part, answer } => {
            driver.submit_answer(year.unwrap(), day.unwrap(), part, answer)
        }
    }
}
