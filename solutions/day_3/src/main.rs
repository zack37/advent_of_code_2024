use regex::Regex;

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "part_1")]
    part_1()?;

    #[cfg(feature = "part_2")]
    part_2()?;

    Ok(())
}

fn parse_input(input: &'static str) -> Vec<(i32, i32)> {
    let RE = Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();
    let mut results: Vec<(i32, i32)> = vec![];
    for (_, [x, y]) in RE.captures_iter(input).map(|c| c.extract()) {
        results.push((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()));
    }

    println!("{results:?}");

    results
}

#[cfg(feature = "part_1")]
fn part_1() -> anyhow::Result<()> {
    println!("Day 3 part 1");
    let input = include_str!("inputs/sample.txt");
    let answer: i32 = parse_input(input).into_iter().map(|(x, y)| x * y).sum();

    println!("Answer: {}", answer);

    Ok(())
}

fn parse_pt2_input(input: &'static str) -> Vec<(i32, i32)> {
    let RE = Regex::new(r#"(do\(\))|(don't\(\))|mul\((\d+),(\d+)\)"#).unwrap();
    let mut results: Vec<(i32, i32)> = vec![];
    let mut enabled = true;
    for matches in RE.captures_iter(input) {
        if matches[0].eq("do()") {
            enabled = true;
        } else if matches[0].eq("don't()") {
            enabled = false;
            println!("disable that shit");
        } else if enabled {
            let x = matches[3].parse::<i32>().unwrap();
            let y = matches[4].parse::<i32>().unwrap();
            results.push((x, y));
        }
        println!("{matches:?}");
        // results.push((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()));
    }

    println!("{results:?}");

    results
}

#[cfg(feature = "part_2")]
fn part_2() -> anyhow::Result<()> {
    println!("Day 3 part 2");
    let input = include_str!("inputs/input.txt");
    let answer: i32 = parse_pt2_input(input).into_iter().map(|(x, y)| x * y).sum();

    println!("Answer: {}", answer);

    Ok(())
}
