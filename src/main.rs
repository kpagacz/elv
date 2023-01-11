use std::{io::Write, path::PathBuf};

use chrono::Datelike;
use clap::Parser;
use config::{builder::DefaultState, ConfigBuilder};
use elv::{CliCommand, CliInterface, Configuration, Driver};

fn main() {
    let cli = CliInterface::parse();

    let mut builder = Configuration::builder();

    if let Some(token) = cli.token {
        builder = builder
            .set_override("aoc.token", token)
            .expect("Failed to set the Advent Of Code token");
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

    match cli.command {
        CliCommand::Input {
            out,
            no_file,
            print,
        } => handle_input_command(builder, year.unwrap(), day.unwrap(), out, no_file, print),
        CliCommand::Submit { part, answer } => {
            let driver = Driver::new(get_configuration(builder));
            match driver.submit_answer(year.unwrap(), day.unwrap(), part, answer) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        CliCommand::ClearCache => handle_clear_cache_command(builder),
        CliCommand::Description { width } => {
            handle_description_command(builder, year.unwrap(), day.unwrap(), width)
        }
        CliCommand::ListDirs => handle_list_dirs_command(builder),
    }

    fn handle_input_command(
        configuration_builder: ConfigBuilder<DefaultState>,
        year: u16,
        day: u8,
        out: PathBuf,
        no_file: bool,
        print: bool,
    ) {
        let driver = Driver::new(get_configuration(configuration_builder));
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
            Err(e) => eprintln!("❗️ Error when getting the input:\n\t{}", e.description()),
        }
    }

    fn handle_clear_cache_command(configuration_builder: ConfigBuilder<DefaultState>) {
        let driver = Driver::new(get_configuration(configuration_builder));
        match driver.clear_cache() {
            Ok(_) => eprintln!("✅ Cache cleared"),
            Err(e) => panic!("❌ error when clearing cache: {}", e.description()),
        }
    }

    fn handle_description_command(
        mut configuration_builder: ConfigBuilder<DefaultState>,
        year: u16,
        day: u8,
        width: Option<usize>,
    ) {
        if let Some(width) = width {
            configuration_builder = configuration_builder
                .set_override("cli.output_width", width as u64)
                .expect("Failed to set the output width");
        }
        let driver = Driver::new(get_configuration(configuration_builder));
        match driver.get_description(year, day) {
            Ok(description) => println!("{}", description),
            Err(e) => panic!("Error when getting the description: {}", e.description()),
        }
    }

    fn handle_list_dirs_command(configuration_builder: ConfigBuilder<DefaultState>) {
        let driver = Driver::new(get_configuration(configuration_builder));
        match driver.list_app_directories() {
            Ok(dirs) => {
                for (name, path) in dirs {
                    println!("{}: {}", name, path);
                }
            }
            Err(e) => panic!("Error when listing the directories: {}", e.description()),
        }
    }

    fn get_configuration(builder: ConfigBuilder<DefaultState>) -> Configuration {
        let configuration = builder
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap_or_else(|_| {
                eprintln!("Failed to deserialize the configuration, using default...");
                Configuration::new()
            });
        configuration
    }
}
