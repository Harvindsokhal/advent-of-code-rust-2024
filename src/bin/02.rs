use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn parse_input<R: BufRead>(reader: R) -> Result<Vec<Vec<i32>>> {
    reader
        .lines()
        .map(|line| {
            line.map_err(|e| anyhow!(e))? // Map IO error to anyhow error
                .split_whitespace()
                .map(|s| s.parse::<i32>().map_err(|e| anyhow!(e))) // Map parse error to anyhow error
                .collect::<Result<Vec<i32>>>() // Collect parsed integers into a Vec<i32>
        })
        .collect::<Result<Vec<Vec<i32>>>>() // Collect all Vec<i32> into a Vec<Vec<i32>>
}

fn is_safe(report: &[i32]) -> bool {
    let diffs: Vec<i32> = report.windows(2).map(|w| w[1] - w[0]).collect();

    let all_increasing = diffs.iter().all(|&d| d > 0 && d <=3);
    let all_decreasing = diffs.iter().all(|&d| d < 0 && d >= -3);

    all_increasing || all_decreasing
}

fn is_safe_with_dampener(levels: &[i32]) -> bool {
    if is_safe(levels) {
        return true; // Already safe without removing any level
    }

    // Try removing each level and checking if the result is safe
    for i in 0..levels.len() {
        let mut modified = levels.to_vec();
        modified.remove(i); // Remove one level
        if is_safe(&modified) {
            return true; // Found a modification that maks the report safe
        }
    }

    false // No single modification can make the report safe
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let reports = parse_input(reader)?;
    let safe_count = reports.iter().filter(|&report| is_safe(report)).count();
    Ok(safe_count)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let reports = parse_input(reader)?;
    let safe_count = reports
        .iter()
        .filter(|report| is_safe_with_dampener(report))
        .count();
    Ok(safe_count)
}

fn main() -> Result<()> {
    start_day(DAY);

    // Part 1
    println!("=== Part 1 ===");
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    // Part 2
    println!("=== Part 2 ===");
    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
