use clap::{arg, Command};
use std::env;
use std::ffi::OsString;

#[derive(PartialEq, Debug)]
pub struct Config {
    pub host: String,
    pub token: String,
}

impl Config {
    pub fn from_args() -> Self {
        Config::build_from(env::args_os())
    }

    fn build_from<I, T>(args: I) -> Self 
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let matches = Command::new("Gitea Bundle")
            .arg_required_else_help(true)
            .about("Gita backup bundler")
            .arg(arg!(<host> "Gitea host"))
            .arg(arg!(<token> "Access token"))
        .get_matches_from(args);

        Config { 
            host: matches.get_one::<String>("host").unwrap().clone(),
            token: matches.get_one::<String>("token").unwrap().clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_test() {
        let cfg = Config::build_from(
            [
                "gitea-bundle",
                "http://example.com",
                "1234567890",
            ]
            .iter()
        );
        assert_eq!(cfg, 
            Config{
                host: String::from("http://example.com"), 
                token: String::from("1234567890")
            }
        );
    }
}
