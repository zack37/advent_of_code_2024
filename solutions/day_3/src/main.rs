use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many_till, many1},
    sequence::{delimited, separated_pair},
};

fn main() -> anyhow::Result<()> {
    let input = include_str!("inputs/input.txt");
    #[cfg(feature = "part_1")]
    {
        let answer = part_1(input)?;
        println!("Part 1: {}", answer)
    }

    #[cfg(feature = "part_2")]
    {
        let answer = part_2(input)?;
        println!("Part 2: {}", answer);
    }

    Ok(())
}

fn mul(input: &'static str) -> IResult<&'static str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::i32, tag(","), complete::i32),
        tag(")"),
    )(input)?;

    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn parse(input: &'static str) -> anyhow::Result<i32> {
    let (_, value) = many1(many_till(anychar, mul).map(|(_discard, ins)| ins))(input)?;

    Ok(value
        .iter()
        .map(|i| match i {
            Instruction::Mul(x, y) => x * y,
            _ => 0,
        })
        .sum())
}

#[cfg(feature = "part_1")]
fn part_1(input: &'static str) -> anyhow::Result<i32> {
    println!("Day 3 part 1");

    parse(input)
}

#[derive(Debug, Clone)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

fn instruction(input: &'static str) -> IResult<&'static str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}

fn parse_pt2_input(input: &'static str) -> anyhow::Result<i32> {
    let (_, value) = many1(many_till(anychar, instruction).map(|(_discard, ins)| ins))(input)?;

    let (_, result) = value
        .iter()
        .fold((true, 0), |(enabled, result), ins| match ins {
            Instruction::Mul(a, b) => {
                if enabled {
                    (enabled, result + a * b)
                } else {
                    (enabled, result)
                }
            }
            Instruction::Do => (true, result),
            Instruction::Dont => (false, result),
        });

    Ok(result)
}

#[cfg(feature = "part_2")]
fn part_2(input: &'static str) -> anyhow::Result<i32> {
    println!("Day 3 part 2");

    parse_pt2_input(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "part_1")]
    fn part_1_test() {
        let input = include_str!("inputs/sample.txt");
        assert_eq!(part_1(input).unwrap(), 161);
    }

    #[test]
    #[cfg(feature = "part_2")]
    fn part_2_test() {
        let input = include_str!("inputs/sample_2.txt");
        assert_eq!(part_2(input).unwrap(), 48);
    }
}
