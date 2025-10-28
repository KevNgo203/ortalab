use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Opts {
    pub file: PathBuf,

    #[arg(long)]
    pub explain: bool,
}
