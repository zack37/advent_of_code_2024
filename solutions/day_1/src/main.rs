use std::collections::HashMap;

fn main() {
    let input = include_str!("inputs/input.txt");

    #[cfg(feature = "part_1")]
    {
        let answer = part_1(input);
        println!("Part 1: {}", answer);
    }
    #[cfg(feature = "part_2")]
    {
        let answer = part_2(input);
        println!("Part 2: {}", answer);
    }
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
fn part_1(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

#[cfg(feature = "part_2")]
fn part_2(input: &str) -> i32 {
    let (left, right) = parse_input(input);
    let mut occurences: HashMap<i32, i32> = HashMap::new();
    for x in right {
        occurences.entry(x).and_modify(|e| *e += 1).or_insert(1);
    }

    left.into_iter()
        .map(|l| l * occurences.get(&l).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "part_1")]
    fn part_1_test() {
        let input = include_str!("inputs/sample.txt");
        assert_eq!(part_1(input), 11);
    }

    #[test]
    #[cfg(feature = "part_2")]
    fn part_2_test() {
        let input = include_str!("inputs/sample.txt");
        assert_eq!(part_2(input), 31);
    }
}
