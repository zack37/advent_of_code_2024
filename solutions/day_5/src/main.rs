use std::{cmp::Ordering, collections::HashMap};

use nom::{
    bytes::complete::tag,
    character::{complete, complete::line_ending},
    multi::{fold_many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
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

fn rules(input: &str) -> IResult<&str, HashMap<u32, Vec<u32>>> {
    fold_many1(
        terminated(
            separated_pair(complete::u32, tag("|"), complete::u32),
            line_ending,
        ),
        HashMap::default,
        |mut acc: HashMap<u32, Vec<u32>>, (page, after)| {
            acc.entry(page)
                .and_modify(|afters| {
                    afters.push(after);
                })
                .or_insert(vec![after]);
            acc
        },
    )(input)
}

fn updates(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(line_ending, separated_list1(tag(","), complete::u32))(input)
}

fn parse(input: &'static str) -> anyhow::Result<(HashMap<u32, Vec<u32>>, Vec<Vec<u32>>)> {
    let (input, parsed_rules) = terminated(rules, line_ending)(input)?;
    let (_, parsed_updates) = updates(input)?;

    Ok((parsed_rules, parsed_updates))
}

fn part_1(input: &'static str) -> anyhow::Result<u32> {
    let (rules, updates) = parse(input)?;

    let results: Vec<usize> = updates
        .iter()
        .enumerate()
        .filter_map(|(index, original_update)| {
            let mut current_item = original_update[0];
            let mut update = &original_update[1..];
            let mut before_pages = &original_update[0..0];

            while before_pages.len() != original_update.len() {
                if let Some(pages_that_must_come_after) = rules.get(&current_item) {
                    if !pages_that_must_come_after
                        .iter()
                        .all(|page| !before_pages.contains(page))
                    {
                        return None;
                    }
                }

                before_pages = &original_update[0..(before_pages.len() + 1)];

                if let Some(page) = update.first() {
                    current_item = *page;
                    update = &update[1..];
                }
            }

            Some(index)
        })
        .collect();

    let result: u32 = results
        .iter()
        .map(|index| {
            let middle = updates[*index].len() / 2;
            updates[*index][middle]
        })
        .sum();

    Ok(result)
}

fn part_2(input: &'static str) -> anyhow::Result<u32> {
    let (rules, updates) = parse(input)?;

    let results: Vec<usize> = updates
        .iter()
        .enumerate()
        .filter_map(|(index, original_update)| {
            let mut current_item = original_update[0];
            let mut update = &original_update[1..];
            let mut before_pages = &original_update[0..0];

            while before_pages.len() != original_update.len() {
                if let Some(pages_that_must_come_after) = rules.get(&current_item) {
                    if !pages_that_must_come_after
                        .iter()
                        .all(|page| !before_pages.contains(page))
                    {
                        return Some(index);
                    }
                }

                before_pages = &original_update[0..(before_pages.len() + 1)];

                if let Some(page) = update.first() {
                    current_item = *page;
                    update = &update[1..];
                }
            }

            None
        })
        .collect();
    let sorted_results: Vec<_> = results
        .iter()
        .map(|index| {
            let mut update = updates[*index].clone();
            update.sort_by(|a, b| {
                if rules.get(a).is_some_and(|pages| pages.contains(b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            update
        })
        .collect();

    let result: u32 = sorted_results
        .iter()
        .map(|result| {
            let middle = result.len() / 2;
            result[middle]
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
        assert_eq!(part_1(include_str!("inputs/sample.txt")).unwrap(), 143);
    }

    #[test]
    #[cfg(feature = "part_2")]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("inputs/sample.txt")).unwrap(), 123);
    }
}
