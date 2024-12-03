use anyhow::Context;
use nom::character::complete::{char, digit1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let input = include_str!("inputs/input.txt");

    #[cfg(feature = "part_1")]
    part_1(input)?;

    #[cfg(feature = "part_2")]
    part_2(input)?;

    Ok(())
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    map(digit1, |d: &str| d.parse::<i32>().unwrap())(input)
}

fn parse_line(line: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(char(' '), parse_number)(line)
}

fn parse_input(input: &'static str) -> anyhow::Result<Vec<Vec<i32>>> {
    let (remaining, pairs) =
        separated_list1(char('\n'), parse_line)(input).context("Unable to parse input")?;

    if !remaining.is_empty() {
        anyhow::bail!("Unexpected remaining input: {:?}", remaining);
    }

    Ok(pairs)
}

#[cfg(feature = "part_1")]
fn part_1(input: &'static str) -> anyhow::Result<()> {
    let parsed = parse_input(input)?;

    let mut safe = 0;
    for line in parsed {
        let mut direction: Option<i32> = None;
        let mut is_safe = true;
        for w in line.windows(2) {
            let diff = w[0] - w[1];
            let diff_abs = diff.abs();
            let dir = if diff == 0 { 0 } else { diff / diff_abs };

            if direction.is_none() {
                direction = Some(dir);
            }

            match direction {
                Some(x) if x != dir => is_safe = false,
                _ => {
                    if is_safe && (diff_abs == 0 || diff_abs > 3) {
                        is_safe = false;
                    }
                }
            };
        }

        safe += if is_safe { 1 } else { 0 };
    }

    println!("Part 1: {}", safe);

    Ok(())
}

enum Direction {
    Asc,
    Desc,
}

fn check_line(arr: &[i32]) -> bool {
    let dir = if arr[0] < arr[1] {
        Direction::Asc
    } else {
        Direction::Desc
    };
    for i in 0..arr.len() - 1 {
        let diff = (arr[i] - arr[i + 1]).abs();
        match dir {
            Direction::Asc => {
                if arr[i] > arr[i + 1] || !(1..=3).contains(&diff) {
                    return false;
                }
            }
            Direction::Desc => {
                if arr[i] < arr[i + 1] || !(1..=3).contains(&diff) {
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(feature = "part_2")]
fn part_2(input: &'static str) -> anyhow::Result<()> {
    let parsed = parse_input(input)?;

    let mut safe = 0;
    for mut line in parsed {
        if check_line(&line) {
            safe += 1;
            continue;
        }
        'dampening: for i in 0..line.len() {
            let mut line = line.clone();
            line.remove(i);
            if check_line(&line) {
                safe += 1;
                break 'dampening;
            }
        }
    }

    println!("Part 2: {}", safe);

    Ok(())
}
