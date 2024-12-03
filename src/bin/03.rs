use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use adv_code_2024::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = r#"
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"#;

fn parse_and_compute<R: BufRead>(reader: R) -> Result<usize> {
    // Regex to match valid `mul(X,Y)` instructions
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        for cap in regex.captures_iter(&line) {
            let x: usize = cap[1].parse()?;
            let y: usize = cap[2].parse()?;
            sum += x * y;
        }
    }
    Ok(sum)
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    parse_and_compute(reader)
}

fn main() -> Result<()> {
    start_day(DAY);

    // Part 1
    println!("=== Part 1 ===");
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);


    Ok(())
}
