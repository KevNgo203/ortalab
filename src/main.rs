use std::error::Error;

use clap::Parser;
use ortalab::{cli::Opts, io::parse_round, poker::score};

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let round = parse_round(&opts)?;

    let (chips, mult) = score(round);

    println!("{}", (chips * mult).floor());
    Ok(())
}
