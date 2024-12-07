use itertools::Itertools;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
};

fn main() -> anyhow::Result<()> {
    let input = include_str!("inputs/input.txt");

    #[cfg(feature = "part_1")]
    {
        let answer = part_1(input)?;
        println!("Part 1: {}", answer);
    }

    #[cfg(feature = "part_2")]
    {
        let answer = part_2(input)?;
        println!("Part 2: {}", answer);
    }

    Ok(())
}

fn parse_input(input: &'static str) -> IResult<&'static str, Vec<(u64, Vec<u64>)>> {
    separated_list1(
        line_ending,
        separated_pair(
            complete::u64,
            tag(": "),
            separated_list1(space1, complete::u64),
        ),
    )(input)
}

#[cfg(feature = "part_1")]
const PART_1_OPERATIONS: [char; 2] = ['+', '*'];
#[cfg(feature = "part_2")]
const PART_2_OPERATIONS: [&str; 3] = ["+", "*", "||"];

#[cfg(feature = "part_1")]
fn part_1(input: &'static str) -> anyhow::Result<u64> {
    let (_, lines) = parse_input(input)?;

    let result: u64 = lines
        .iter()
        .filter_map(|(test, numbers)| {
            let operator_count = numbers.len() - 1;
            (0..operator_count)
                .map(|_| PART_1_OPERATIONS)
                .multi_cartesian_product()
                .any(|seq| {
                    let mut s = seq.iter();
                    let result = numbers
                        .iter()
                        .copied()
                        .reduce(|acc, cur| match *s.next().unwrap() {
                            '*' => acc * cur,
                            '+' => acc + cur,
                            _ => panic!("Invalid operator"),
                        })
                        .unwrap();
                    *test == result
                })
                .then_some(test)
        })
        .sum();

    Ok(result)
}

#[cfg(feature = "part_2")]
fn part_2(input: &'static str) -> anyhow::Result<u64> {
    let (_, lines) = parse_input(input)?;

    let result: u64 = lines
        .iter()
        .filter_map(|(test, numbers)| {
            let operator_count = numbers.len() - 1;
            (0..operator_count)
                .map(|_| PART_2_OPERATIONS)
                .multi_cartesian_product()
                .any(|seq| {
                    let mut s = seq.iter();
                    let result = numbers
                        .iter()
                        .copied()
                        .reduce(|acc, cur| match *s.next().unwrap() {
                            "*" => acc * cur,
                            "+" => acc + cur,
                            "||" => format!("{acc}{cur}").parse::<u64>().unwrap(),
                            _ => panic!("Invalid operator"),
                        })
                        .unwrap();
                    *test == result
                })
                .then_some(test)
        })
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "part_1")]
    fn test_part_1() {
        let input = include_str!("inputs/sample.txt");
        let answer = part_1(input).unwrap();
        assert_eq!(answer, 3749);
    }

    #[test]
    #[cfg(feature = "part_2")]
    fn test_part_2() {
        let input = include_str!("inputs/sample.txt");
        let answer = part_2(input).unwrap();
        assert_eq!(answer, 11387);
    }
}
