use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "part_1")]
    part_1()?;

    #[cfg(feature = "part_2")]
    part_2()?;

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
fn part_1() -> anyhow::Result<()> {
    println!("Day 3 part 1");
    let input = include_str!("inputs/sample.txt");
    let answer: i32 = parse(input)?;

    println!("Answer: {}", answer);

    Ok(())
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
fn part_2() -> anyhow::Result<()> {
    println!("Day 3 part 2");
    let input = include_str!("inputs/input.txt");
    let answer = parse_pt2_input(input)?;

    println!("Answer: {}", answer);

    Ok(())
}
