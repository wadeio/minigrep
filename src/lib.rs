use std::env;
use std::fs;
use std::io::{self, Read};
use thiserror::Error;

pub fn run(config: Config) {
    let Config {
        mut query,
        mut contents,
        ignore_case,
    } = config;

    if ignore_case {
        contents = contents.to_lowercase();
        query = query.to_lowercase();
    }
    search(&query, &contents).for_each(|line| println!("{line}"));
}

pub struct Config {
    query: String,
    contents: String,
    ignore_case: bool,
}

#[derive(Error, Debug)]
pub enum ConfigBuildError {
    #[error("not enough arguments")]
    NotEnoughArgs,

    #[error("IO Error, error: {}", .source)]
    IOError {
        #[from]
        source: io::Error,
    },

    #[error("{}", .msg)]
    FileError { msg: String },
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, ConfigBuildError> {
        args.next();

        let query = args.next().ok_or(ConfigBuildError::NotEnoughArgs)?;

        let contents = if let Some(file_path) = args.next() {
            fs::read_to_string(&file_path).map_err(|err| ConfigBuildError::FileError {
                msg: format!("Fail to read file, path: {file_path}, error: {err}"),
            })?
        } else {
            let mut stdin = String::new();

            io::stdin().read_to_string(&mut stdin)?;

            stdin
        };

        let ignore_case = env::var("MINIGREP_IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            contents,
            ignore_case,
        })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents.lines().filter(move |line| line.contains(query))
}

#[cfg(test)]
mod tests;
