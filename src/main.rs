use clap::Parser;
use ortalab::{cli::Opts, io::parse_round, poker::score};
use std::error::Error;

/// Entry point of the OrtaLab CLI.
///
/// This function:
/// - Parses command-line arguments into [`Opts`].
/// - Reads and parses a poker round from input.
/// - Computes the chip score and multiplier.
/// - Prints the final floored chip value.
///
/// # Errors
/// Returns an error if parsing the round fails.
///
/// # Example
/// ```no_run
/// use ortalab::{cli::Opts, io::parse_round, poker::score};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let opts = Opts::parse();
/// let round = parse_round(&opts)?;
/// let (chips, mult) = score(round);
/// println!("{}", (chips * mult).floor());
/// # Ok(())
/// # }
/// ```
fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let round = parse_round(&opts)?;

    let (chips, mult) = score(round);

    println!("{}", (chips * mult).floor());
    Ok(())
}
