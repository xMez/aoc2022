use std::fs::File;
use std::io::{self, Error, Read, Write};

use clap::Parser;

pub fn calorie_counting(input: &mut impl Read, output: &mut impl Write) -> Result<(), Error> {
    let mut buffer = "".to_string();

    input.read_to_string(&mut buffer)?;
    let calories: Vec<u32> = buffer
        .split_terminator("\n")
        .map(|x| x.parse().unwrap_or(0))
        .collect();
    let v: Vec<_> = calories
        .split(|&e| e == 0)
        .map(|x| x.iter().sum::<u32>())
        .collect();
    let index_of_max: usize = v
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| (*a).cmp(*b))
        .map(|(index, _)| index)
        .unwrap();
    output.write_fmt(format_args!(
        "Elf {} is carrying the most Calories with {}.",
        index_of_max,
        v.get(index_of_max).unwrap()
    ))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_upcased_input_to_output() {
        let mut output: Vec<u8> = Vec::new();

        calorie_counting(&mut "2000\n2000\n\n3000".as_bytes(), &mut output).unwrap();
        assert_eq!(output, b"Elf 0 is carrying the most Calories with 4000.");
    }
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
    calorie_counting(&mut input, &mut output)
    // calorie_counting(&mut io::stdin(), &mut io::stdout())
}
