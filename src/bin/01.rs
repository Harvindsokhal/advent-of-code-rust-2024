use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
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

fn part2<R: BufRead>(reader: R) -> Result<i32> {
    let (left, right) = parse_input(reader)?;

    // Count occurrences in the right list
    let mut right_counts: HashMap<i32, i32> = HashMap::new();
    for &num in &right {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    let similarity_score: i32 = left
        .iter()
        .map(|&num| num * right_counts.get(&num).copied().unwrap_or(0))
        .sum();

    Ok(similarity_score)
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

    println!("=== Part 2 ===");
    assert_eq!(
        31,
        part2(BufReader::new(TEST.as_bytes())).expect("Test case failed")
    );

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part2(input_file)?;
    println!("Result = {}", result);

   Ok(())
}
