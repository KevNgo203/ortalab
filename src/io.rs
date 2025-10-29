use std::{
    error::Error,
    fs::File,
    io::{Read, stdin},
    path::Path,
};

use crate::cli::Opts;
use ortalib::Round;

/// Parses a poker round from the given CLI options.
///
/// # Arguments
/// * `opts` - Command-line options specifying the input source.
///
/// # Returns
/// A parsed `Round` structure representing the poker game state.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
///
/// # Example
/// ```no_run
/// use ortalab::{cli::Opts, io::parse_round};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let opts = Opts::parse();
/// let round = parse_round(&opts)?;
/// # Ok(())
/// # }
/// ```
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
