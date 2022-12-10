use std::fs::File;
use std::io::{self, Error, Read, Write};

use clap::Parser;

pub fn solve(hand: &str) -> u16 {
    match hand {
        // Rock
        "A X" => 3 + 1, // Rock v Rock
        "A Y" => 6 + 2, // Rock v Paper
        "A Z" => 0 + 3, // Rock v Scissors
        // Paper
        "B X" => 0 + 1, // Paper v Rock
        "B Y" => 3 + 2, // Paper v Paper
        "B Z" => 6 + 3, // Paper v Scissors
        // Scissors
        "C X" => 6 + 1, // Scissors v Rock
        "C Y" => 0 + 2, // Scissors v Paper
        "C Z" => 3 + 3, // Scissors v Scissors
        // Empty
        "" => 0,
        &_ => todo!(),
    }
}

pub fn secret_solve(hand: &str) -> u16 {
    match hand {
        // Lose
        "A X" => 0 + 3, // Rock v Scissors
        "B X" => 0 + 1, // Paper v Rock
        "C X" => 0 + 2, // Scissors v Paper
        // Draw
        "A Y" => 3 + 1, // Rock v Rock
        "B Y" => 3 + 2, // Paper v Paper
        "C Y" => 3 + 3, // Scissors v Scissors
        // Win
        "A Z" => 6 + 2, // Rock v Paper
        "B Z" => 6 + 3, // Paper v Scissors
        "C Z" => 6 + 1, // Scissors v Rock
        // Empty
        "" => 0,
        &_ => todo!(),
    }
}

pub fn rock_paper_scissors(input: &mut impl Read, output: &mut impl Write) -> Result<(), Error> {
    let mut buffer = "".to_string();

    input.read_to_string(&mut buffer)?;
    output.write_fmt(format_args!(
        "Part 1 score: {}\nPart 2 score: {}\n",
        buffer.lines().map(|x| solve(x)).sum::<u16>(),
        buffer.lines().map(|x| secret_solve(x)).sum::<u16>()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver() {
        let mut result = solve("A Y");
        assert_eq!(result, 8);
        result = solve("B X");
        assert_eq!(result, 1);
        result = solve("C Z");
        assert_eq!(result, 6)
    }

    #[test]
    fn test_rock_paper_scissors() {
        let mut output: Vec<u8> = Vec::new();
        rock_paper_scissors(&mut "A Y\nB X\nC Z\n".as_bytes(), &mut output).unwrap();
        assert_eq!(output, b"Part 1 score: 15\nPart 2 score: 12\n")
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
    rock_paper_scissors(&mut input, &mut output)
}
