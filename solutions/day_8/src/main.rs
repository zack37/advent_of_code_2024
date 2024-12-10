use std::iter::successors;

use glam::IVec2;
use itertools::Itertools;
use rayon::prelude::*;

fn main() -> anyhow::Result<()> {
    let input = include_str!("inputs/input.txt");

    #[cfg(feature = "part_1")]
    println!("Part 1: {}", part_1(input)?);

    #[cfg(feature = "part_2")]
    println!("Part 2: {}", part_2(input)?);

    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Antenna {
    label: char,
    pos: IVec2,
}

fn parse_input(input: &'static str) -> Vec<Antenna> {
    input
        .lines()
        .enumerate()
        .flat_map(move |(row, l)| {
            l.chars().enumerate().filter_map(move |(col, c)| {
                if c == '.' {
                    None
                } else {
                    Some(Antenna {
                        label: c,
                        pos: IVec2::new(row as i32, col as i32),
                    })
                }
            })
        })
        .collect()
}

#[cfg(feature = "part_1")]
fn part_1(input: &'static str) -> anyhow::Result<usize> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut antenna = parse_input(input);
    let y_bounds = 0..height as i32;
    let x_bounds = 0..width as i32;

    antenna.sort_by(|a, b| a.label.cmp(&b.label));

    let result = antenna
        .chunk_by(|a, b| a.label == b.label)
        .flat_map(|chunk| {
            chunk.iter().combinations(2).flat_map(|c| {
                let diff = c[0].pos - c[1].pos;
                [c[0].pos + diff, c[1].pos - diff]
            })
        })
        .filter(|pos| x_bounds.contains(&pos.x) && y_bounds.contains(&pos.y))
        .unique()
        .count();

    Ok(result)
}

#[cfg(feature = "part_2")]
fn part_2(input: &'static str) -> anyhow::Result<usize> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut antenna = parse_input(input);
    let y_bounds = 0..height as i32;
    let x_bounds = 0..width as i32;

    antenna.sort_by(|a, b| a.label.cmp(&b.label));

    let result = antenna
        .chunk_by(|a, b| a.label == b.label)
        .flat_map(|chunk| {
            chunk
                .iter()
                .combinations(2)
                .flat_map(|c| {
                    let diff = c[0].pos - c[1].pos;

                    let first: Vec<_> = successors(Some(c[0].pos), |pos| {
                        let new_pos = pos + diff;
                        if x_bounds.contains(&pos.x) && y_bounds.contains(&pos.y) {
                            Some(new_pos)
                        } else {
                            None
                        }
                    })
                    .collect();

                    let second = successors(Some(c[1].pos), |pos| {
                        let new_pos = pos - diff;
                        if x_bounds.contains(&pos.x) && y_bounds.contains(&pos.y) {
                            Some(new_pos)
                        } else {
                            None
                        }
                    })
                    .collect();

                    [first, second]
                })
                .flatten()
        })
        .filter(|pos| x_bounds.contains(&pos.x) && y_bounds.contains(&pos.y))
        .unique()
        .count();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "part_1")]
    fn part_1_test() {
        let input = include_str!("inputs/sample.txt");
        let answer = part_1(input).unwrap();

        assert_eq!(answer, 14)
    }

    #[test]
    #[cfg(feature = "part_2")]
    fn part_2_test() {
        let input = include_str!("inputs/sample.txt");
        let answer = part_2(input).unwrap();

        assert_eq!(answer, 34)
    }
}
