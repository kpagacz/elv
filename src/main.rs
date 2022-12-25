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
            .expect("Failed to set the Advent Of Code token");
        configuration = builder
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap_or_else(|_| {
                eprintln!("Failed to deserialize the configuration, using default...");
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
        CliCommand::Description => handle_description_command(&driver, year.unwrap(), day.unwrap()),
        CliCommand::ListDirs => handle_list_dirs_command(&driver),
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
                if print {
                    println!("{}", input);
                }
                if !no_file {
                    if let Some(parent) = out.parent() {
                        std::fs::create_dir_all(parent).expect(
                            format!("Failed to create the directory {}\nYou can still get the input if you print it with the --print flag",
                            out.to_str().unwrap())
                                .as_str()
                        );
                    }

                    let mut file = std::fs::File::create(&out).expect(
                        format!("Failed to create the file `{}`\nYou can still get the input if you print it with the --print flag",
                        out.to_str().unwrap()).as_str()
                    );

                    match file.write_all(input.as_bytes()) {
                        Ok(_) => eprintln!("✅ Input written to `{}`", out.to_str().unwrap()),
                        Err(_) => panic!(concat!(
                            "❌ Failed to write the input to the file. ",
                            "You can still get the input if you print it with the --print flag"
                        )),
                    }
                }
            }
            Err(e) => panic!("Error when getting the input: {}", e.description()),
        }
    }

    fn handle_clear_cache_command(driver: &Driver) {
        match driver.clear_cache() {
            Ok(_) => eprintln!("✅ Cache cleared"),
            Err(e) => panic!("❌ error when clearing cache: {}", e.description()),
        }
    }

    fn handle_description_command(driver: &Driver, year: u16, day: u8) {
        match driver.get_description(year, day) {
            Ok(description) => println!("{}", description),
            Err(e) => panic!("Error when getting the description: {}", e.description()),
        }
    }

    fn handle_list_dirs_command(driver: &Driver) {
        match driver.list_app_directories() {
            Ok(dirs) => {
                for (name, path) in dirs {
                    println!("{}: {}", name, path);
                }
            }
            Err(e) => panic!("Error when listing the directories: {}", e.description()),
        }
    }
}
