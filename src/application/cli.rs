mod cli_command;
mod cli_config_subcommand;
mod cli_interface;

use std::io::Write;
use std::path::PathBuf;

use anyhow::{Context, Result};
use chrono::Datelike;
use clap::Parser;

use crate::application::cli::{
    cli_command::{CliCommand, RiddleArgs, TokenArgs},
    cli_config_subcommand::ConfigSubcommand,
    cli_interface::CliInterface,
};
use crate::domain::{riddle_date::RiddleDate, riddle_part::RiddlePart};
use crate::infrastructure::cli_display::CliDisplay;
use crate::{Configuration, Driver};

pub struct ElvCli {}

impl ElvCli {
    pub fn run() {
        let cli = CliInterface::parse();

        match cli.command {
            CliCommand::Input {
                riddle_args,
                token_args,
                out,
                no_file,
                print,
            } => handle_input_command(riddle_args, token_args, out, no_file, print),
            CliCommand::Submit {
                riddle_args,
                token_args,
                part,
                answer,
            } => {
                handle_submit_command(riddle_args, token_args, part, answer);
            }
            CliCommand::Description {
                token,
                riddle_args,
                width,
            } => handle_description_command(token, riddle_args, width),
            CliCommand::Leaderboard { token_args, year } => {
                handle_get_leaderboard(token_args, year)
            }
            CliCommand::PrivateLeaderboard {
                token_args,
                leaderboard_id,
                year,
            } => handle_get_private_leaderboard(token_args, &leaderboard_id, year),
            CliCommand::Stars { year } => handle_get_stars(year),
            CliCommand::ClearCache => handle_clear_cache_command(),
            CliCommand::ListDirs => handle_list_dirs_command(),
            CliCommand::Config { cmd } => match cmd {
                ConfigSubcommand::List {} => handle_get_config(),
                ConfigSubcommand::Set { key, value } => handle_set_config(&key, value),
            },
            CliCommand::Token { token } => handle_token_comamand(token),
        }

        fn handle_submit_command(
            riddle_args: RiddleArgs,
            token_args: TokenArgs,
            mut part: Option<RiddlePart>,
            answer: String,
        ) {
            let driver = get_driver(Some(token_args), None);
            let (year, day) = match determine_date(riddle_args) {
                Ok(res) => res,
                Err(e) => {
                    eprintln!("❌ {e:#}");
                    return;
                }
            };
            if part.is_none() {
                part = Some(driver.guess_riddle_part(year, day).context(
                    "❌ Could not guess the riddle part. Provide it manually as an argument",
                ).unwrap());
            }
            match driver.submit_answer(year, day, part.unwrap(), answer) {
                Ok(_) => {}
                Err(e) => eprint!("❌ Failed to submit the answer. {e:#}"),
            }
        }

        fn handle_input_command(
            riddle_args: RiddleArgs,
            token_args: TokenArgs,
            out: PathBuf,
            no_file: bool,
            print: bool,
        ) {
            let driver = get_driver(Some(token_args), None);
            let (year, day) = match determine_date(riddle_args) {
                Ok(res) => res,
                Err(e) => {
                    eprintln!("❌ {e:#}");
                    return;
                }
            };
            match driver.input(year, day) {
                Ok(input) => {
                    if print {
                        println!("{input}");
                    }
                    if !no_file {
                        if let Some(parent) = out.parent() {
                            match std::fs::create_dir_all(parent) {
                                Ok(()) => {}
                                Err(_) => {
                                    panic!(
                                    "Failed to create the directory {}\n \
                                    You can still get the input if you print it with the --print flag", 
                                    out.to_str().unwrap()
                                );
                                }
                            }
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
                Err(e) => eprintln!("❌ Error when getting the input:\n\t{:#}", e),
            }
        }

        fn handle_description_command(
            token_args: TokenArgs,
            riddle_args: RiddleArgs,
            width: usize,
        ) {
            let driver = get_driver(Some(token_args), Some(width));
            let (year, day) = match determine_date(riddle_args) {
                Ok(res) => res,
                Err(e) => {
                    eprintln!("❌ {e:#}");
                    return;
                }
            };
            match driver.get_description(year, day) {
                Ok(description) => println!("{}", description),
                Err(e) => eprintln!("Error when getting the description: {e:#}"),
            }
        }
        fn handle_clear_cache_command() {
            let driver = get_driver(None, None);
            match driver.clear_cache() {
                Ok(_) => eprintln!("✅ Cache cleared"),
                Err(e) => panic!("❌ error when clearing cache: {e:#}"),
            }
        }

        fn handle_list_dirs_command() {
            let driver = get_driver(None, None);
            match driver.list_app_directories() {
                Ok(dirs) => {
                    for (name, path) in dirs {
                        println!("{}: {}", name, path);
                    }
                }
                Err(e) => eprintln!("❌ Error when listing the directories: {e:#}"),
            }
        }

        fn handle_get_leaderboard(token_args: TokenArgs, year: Option<i32>) {
            let driver = get_driver(Some(token_args), None);
            match driver.get_leaderboard(year.unwrap_or_else(determine_year)) {
                Ok(text) => println!("{text}"),
                Err(e) => eprintln!("❌ Error when getting the leaderboards: {e:#}"),
            }
        }

        fn handle_get_private_leaderboard(
            token_args: TokenArgs,
            leaderboard_id: &str,
            year: Option<i32>,
        ) {
            let driver = get_driver(Some(token_args), None);
            let year = year.unwrap_or_else(determine_year);
            match driver.get_private_leaderboard(leaderboard_id, year) {
                Ok(private_leaderboard) => {
                    println!("{}", private_leaderboard.cli_fmt(&driver.configuration))
                }
                Err(e) => eprintln!("❌ {:?}", e),
            }
        }

        fn handle_get_stars(year: Option<i32>) {
            let driver = get_driver(None, None);
            match driver.get_stars(year.unwrap_or_else(determine_year)) {
                Ok(stars) => println!("{}", stars),
                Err(e) => eprintln!("❌ Failure: {e:#}"),
            }
        }

        fn handle_get_config() {
            match Driver::get_config_map() {
                Ok(map) => map
                    .iter()
                    .for_each(|(key, value)| println!("{} {}", key, value)),
                Err(e) => eprintln!("❌ {e:#}"),
            }
        }

        fn handle_set_config(key: &str, value: String) {
            match Driver::set_config_key(key, value) {
                Ok(_) => println!("✅ Key {key} successfully updated"),
                Err(e) => eprintln!("❌ Failure: {e:#}"),
            }
        }

        fn handle_token_comamand(token: Option<String>) {
            match token {
                Some(token) => handle_set_config("aoc.token", token),
                None => {
                    let config = Configuration::new();
                    println!("✅ Your saved token is: {}", config.aoc.token);
                    println!("If you want to update your token, use elv t <YOUR_NEW_TOKEN>");
                }
            }
        }

        fn determine_date(riddle_args: RiddleArgs) -> Result<(usize, usize)> {
            let est_now = chrono::Utc::now() - chrono::Duration::hours(4);
            let best_guess_date =
                RiddleDate::best_guess(riddle_args.year, riddle_args.day, est_now)?;
            Ok((best_guess_date.year, best_guess_date.day))
        }

        fn determine_year() -> i32 {
            let est_now = chrono::Utc::now() - chrono::Duration::hours(4);
            if est_now.month() == 12 {
                est_now.year()
            } else {
                est_now.year() - 1
            }
        }

        fn build_configuration(
            token_args: Option<TokenArgs>,
            terminal_width: Option<usize>,
        ) -> Result<Configuration, anyhow::Error> {
            let mut config_builder = Configuration::builder();

            if let Some(token) = token_args.and_then(|args| args.token) {
                config_builder = config_builder
                    .set_override("aoc.token", token)
                    .context("❌ Failed to set the override on the AOC token")?;
            }

            config_builder = config_builder
                .set_override_option("cli.output_width", terminal_width.map(|width| width as u32))
                .expect("❌ Failed to set the cli output width");

            config_builder
                .build()?
                .try_deserialize::<Configuration>()
                .or(Ok(Configuration::new()))
        }

        fn get_driver(token_args: Option<TokenArgs>, terminal_width: Option<usize>) -> Driver {
            Driver::new(
                build_configuration(token_args, terminal_width)
                    .expect("❌ Failed to build the configuration for the applciation"),
            )
        }
    }
}
