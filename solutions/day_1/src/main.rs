use std::collections::HashMap;

fn main() {
    let input = include_str!("inputs/input.txt");

    #[cfg(feature = "part_1")]
    part_1(input);
    #[cfg(feature = "part_2")]
    part_2(input);
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = vec![];
    let mut right = vec![];
    for mut line in input.lines().map(|l| l.split("   ")) {
        left.push(line.next().unwrap().parse::<i32>().unwrap());
        right.push(line.next().unwrap().parse::<i32>().unwrap());
    }

    (left, right)
}

#[cfg(feature = "part_1")]
fn part_1(input: &str) {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();

    let answer: i32 = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| (l - r).abs())
        .sum();
    println!("Hello, part 1: {answer}");
}

#[cfg(feature = "part_2")]
fn part_2(input: &str) {
    let (left, right) = parse_input(input);
    let mut occurences: HashMap<i32, i32> = HashMap::new();
    for x in right {
        occurences.entry(x).and_modify(|e| *e += 1).or_insert(1);
    }

    let answer: i32 = left
        .into_iter()
        .map(|l| l * occurences.get(&l).unwrap_or(&0))
        .sum();

    println!("Hello, part 2: {answer}");
}
