use std::io::Read;

use super::{AocApi, AOC_URL};
use crate::domain::ports::get_input::GetInput;
use anyhow::{Context, Result};

impl GetInput for AocApi {
    fn get_input(&mut self, day: usize, year: usize) -> Result<String> {
        let url = reqwest::Url::parse(&format!("{}/{}/day/{}/input", AOC_URL, year, day)).context(
            format!(
                "Failed to parse the url: {}/{}/day/{}/input",
                AOC_URL, year, day
            ),
        )?;
        let description = self
            .http_client
            .get(url)
            .send()
            .context(
                "Failed to send the request to the AOC server. Is your internet connection OK?",
            )
            .and_then(|response| {
                response
                    .error_for_status()
                    .context("Got a non-200 response from a server. Is your token up to date?")
            })
            .and_then(|mut ok_response| {
                let mut body = String::new();
                ok_response
                    .read_to_string(&mut body)
                    .context("Failed to read the response body")?;
                if body.starts_with("Please don't repeatedly request this") {
                    anyhow::bail!("You have to wait for the input to be available");
                }
                Ok(body)
            });
        if let Ok(text) = &description {
            self.description = Some((day, year, text.clone()));
        }
        description
    }
}
