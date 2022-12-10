use std::fs::File;
use std::io::{self, Error, Read, Write};

use clap::Parser;

pub fn sort_calories(input: &mut impl Read) -> Result<Vec<u32>, Error> {
    let mut buffer = "".to_string();

    input.read_to_string(&mut buffer)?;
    let raw_data: Vec<u32> = buffer
        .split_terminator("\n")
        .map(|x| x.parse().unwrap_or(0))
        .collect();
    let mut calories: Vec<u32> = raw_data
        .split(|&e| e == 0)
        .map(|x| x.iter().sum::<u32>())
        .collect();
    calories.sort_by(|a, b| b.cmp(a));
    Ok(calories)
}

pub fn sum_top_elves(calories: &Vec<u32>, elves: usize) -> u32 {
    let sum: u32 = calories[..elves].iter().sum();
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted_calories() {
        let sorted = sort_calories(&mut "3000\n\n2000\n2000".as_bytes()).unwrap();
        assert_eq!(sorted, vec![4000, 3000]);
    }

    #[test]
    fn sum_elves() {
        let sum = sum_top_elves(&vec![1, 2, 3], 2);
        assert_eq!(sum, 3);
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
    let calories = sort_calories(&mut input).unwrap();
    output.write_fmt(format_args!(
        "Nr 1: {}\nTop 3: {}\n",
        sum_top_elves(&calories, 1),
        sum_top_elves(&calories, 3)
    ))
}
