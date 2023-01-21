use crate::Configuration;

pub trait CliDisplay {
    fn cli_fmt(&self, configuration: &Configuration) -> String;
}
