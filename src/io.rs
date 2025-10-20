use std::{
    error::Error,
    fs::File,
    io::{Read, stdin},
    path::Path,
};

use crate::cli::Opts;
use ortalib::Round;

pub fn parse_round(opts: &Opts) -> Result<Round, Box<dyn Error>> {
    let mut input = String::new();
    if opts.file == Path::new("-") {
        stdin().read_to_string(&mut input)?;
    } else {
        File::open(&opts.file)?.read_to_string(&mut input)?;
    }

    let round = serde_yaml::from_str(&input)?;
    Ok(round)
}