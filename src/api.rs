use crate::{domain::riddle_part::RiddlePart, Configuration, Driver};
use anyhow::Result;

/// Downloads the input from Advent of Code servers
///
/// # Arguments
///
/// * `day` - the day of the challenge. [1 - 25]
/// * `year` - the year of the challenge. E.g. 2023
/// * `token` - optionally, the token used to authenticate you against AOC servers
///
/// # Token
///
/// You need the token to authenticate against the AOC servers. This function will not work
/// without it. You can pass it directly to this function or set it via one of the other methods.
/// See [the README](https://github.com/kpagacz/elv#faq) for more information.
///
/// If you set the token using the CLI or in the configuration file, this function will reuse
/// it and you will not need to additionally pass the token to the function.
///
/// # Examples
///
/// ```
/// use elv::get_input;
/// fn download_input() -> String {
///     // Will succeed if your token is set using another way
///     get_input(1, 2023, None).unwrap()
/// }
/// fn download_input_with_token() -> String {
///     // No need to set the token in any other way.
///     get_input(1, 2023, Some("123456yourtoken")).unwrap()
/// }
/// ```
pub fn get_input(day: usize, year: usize, token: Option<&str>) -> Result<String> {
    let mut config = Configuration::new();
    if let Some(token) = token {
        config.aoc.token = token.to_owned();
    }

    let driver = Driver::new(config);
    driver.input(year, day)
}

/// Submits an answer to Advent of Code servers
///
/// # Arguments
///
/// * `day` - the day of the challenge. [1 - 25]
/// * `year` - the year of the challenge. E.g. 2023
/// * `answer` - the submitted answer
/// * `riddle_part` - either 1 or 2 indicating, respectively, part one and two of the riddle
/// * `token` - optionally, the token used to authenticate you against AOC servers
///
/// # Examples
///
/// ```
/// use elv::submit;
/// fn submit_answer(answer: &str) {
///     // Submits answer `12344` to the first part of thefirst day of the 2023 AOC.
///     // This invocation will not work if you do not supply the token
///     // some other way.
///     submit(1, 2023, "12344", 1, None).unwrap();
///     // Submits answer `something` to the second part of the 20th day of the 2019 challenge.
///     // This invocation does not need the token set any other way.
///     submit(20, 2019, "something", 2, Some("Mytoken")).unwrap();
/// }
/// ```
pub fn submit(
    day: usize,
    year: usize,
    answer: &str,
    riddle_part: u8,
    token: Option<&str>,
) -> Result<()> {
    let mut config = Configuration::new();
    if let Some(token) = token {
        config.aoc.token = token.to_owned();
    }

    let driver = Driver::new(config);
    let part = match riddle_part {
        1 => RiddlePart::One,
        2 => RiddlePart::Two,
        _ => RiddlePart::One,
    };
    driver.submit_answer(year, day, part, answer.to_owned())?;
    Ok(())
}
