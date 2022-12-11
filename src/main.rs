use std::{io::Write, path::PathBuf};

use chrono::Datelike;
use clap::Parser;
use elv::{CliCommand, CliInterface, Configuration, Driver};

fn main() {
    let cli = CliInterface::parse();

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
        CliCommand::Input {
            out,
            no_file,
            print,
        } => handle_input_command(&driver, year.unwrap(), day.unwrap(), out, no_file, print),
        CliCommand::Submit { part, answer } => {
            driver.submit_answer(year.unwrap(), day.unwrap(), part, answer)
        }
        CliCommand::ClearCache => handle_clear_cache_command(&driver),
    }

    fn handle_input_command(
        driver: &Driver,
        year: u16,
        day: u8,
        out: PathBuf,
        no_file: bool,
        print: bool,
    ) {
        match driver.input(year, day) {
            Ok(input) => {
                if print { println!("{}", input); }
                if !no_file {
                    let mut file = std::fs::File::create(out).expect("Failed to create file");
                    file.write_all(input.as_bytes()).expect("Failed to write to file");
                }
            }
            Err(e) => println!("Error: {}", e.description()),
        }
    }

    fn handle_clear_cache_command(driver: &Driver) {
        match driver.clear_cache() {
            Ok(_) => println!("Cache cleared"),
            Err(e) => println!("Error: {}", e.description()),
        }
    }
}
