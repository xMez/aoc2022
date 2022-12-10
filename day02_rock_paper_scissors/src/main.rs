use std::fs::File;
use std::io::{self, Error, Read, Write};

use clap::Parser;

#[cfg(test)]
mod tests {
    use super::*;
}

#[derive(Parser)]
struct Io {
    #[arg(short, long, default_value = "input")]
    file: std::path::PathBuf,
    #[arg(short, long, default_value = "output")]
    out: std::path::PathBuf,
}

fn main() -> io::Result<()> {
    let args = Io::parse();
    let mut input = File::open(args.file).expect("Unable to open file");
    let mut output = File::create(args.out).expect("Unable to create file");
    todo!();
}
