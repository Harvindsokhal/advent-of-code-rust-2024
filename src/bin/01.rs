use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
const TEST: &str = "\
3 4
4 3
2 5
1 3
3 9
3 3
"; // TODO: Add the test input

fn parse_input<R: BufRead>(reader: R) -> Result<(Vec<i32>, Vec<i32>)> {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        if let (Some(left_val), Some(right_val)) = (parts.next(), parts.next()) {
            left.push(left_val.parse()?);
            right.push(right_val.parse()?);
        }
    }
    Ok((left, right))
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let (mut left, mut right) = parse_input(reader)?;

    // Sort the lists
    left.sort_unstable();
    right.sort_unstable();

    // Calculate the total distance
    let total_distance: usize = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs() as usize)
        .sum();

    Ok(total_distance)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    assert_eq!(
        11,
        part1(BufReader::new(TEST.as_bytes())).expect("Test case failed")
    );

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
