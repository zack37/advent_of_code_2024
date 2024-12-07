use std::{collections::HashSet, fmt::Formatter};

use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::value,
    multi::{many1, separated_list1},
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

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Piece {
    Open,
    Obstruction,
    Guard(Direction),
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::Open => write!(f, "."),
            Piece::Obstruction => write!(f, "#"),
            Piece::Guard(direction) => write!(f, "{}", direction),
        }
    }
}

fn find_guard(grid: &[Vec<Piece>]) -> Option<((usize, usize), Piece)> {
    for (row, _) in grid.iter().enumerate() {
        for (col, piece) in grid[row].iter().enumerate() {
            if let Piece::Guard(_) = piece {
                return Some(((row, col), piece.clone()));
            }
        }
    }

    None
}

fn parse_line(input: &str) -> IResult<&str, Vec<Piece>> {
    many1(alt((
        value(Piece::Open, tag(".")),
        value(Piece::Obstruction, tag("#")),
        value(Piece::Guard(Direction::Up), tag("^")),
        value(Piece::Guard(Direction::Down), tag("v")),
        value(Piece::Guard(Direction::Right), tag(">")),
        value(Piece::Guard(Direction::Left), tag("<")),
    )))(input)
}

fn parse_input(input: &'static str) -> anyhow::Result<Vec<Vec<Piece>>> {
    let (_input, res) = separated_list1(newline, parse_line)(input)?;
    Ok(res)
}

#[cfg(feature = "part_1")]
fn part_1(input: &'static str) -> anyhow::Result<usize> {
    let grid = parse_input(input)?;
    let Some(((mut row, mut col), Piece::Guard(mut direction))) = find_guard(&grid) else {
        anyhow::bail!("Where is the guard?");
    };
    let mut seen: HashSet<(usize, usize)> = HashSet::default();
    loop {
        match direction {
            Direction::Up => {
                if row == 0 {
                    return Ok(seen.len());
                }

                if grid[row - 1][col] == Piece::Obstruction {
                    direction = Direction::Right;
                } else {
                    row -= 1;
                    seen.insert((row, col));
                }
            }
            Direction::Down => {
                if row == (grid.len() - 1) {
                    return Ok(seen.len());
                }

                if grid[row + 1][col] == Piece::Obstruction {
                    direction = Direction::Left;
                } else {
                    row += 1;
                    seen.insert((row, col));
                }
            }
            Direction::Left => {
                if col == 0 {
                    return Ok(seen.len());
                }
                if grid[row][col - 1] == Piece::Obstruction {
                    direction = Direction::Up;
                } else {
                    col -= 1;
                    seen.insert((row, col));
                }
            }
            Direction::Right => {
                if col == (grid.len() - 1) {
                    return Ok(seen.len());
                }

                if grid[row][col + 1] == Piece::Obstruction {
                    direction = Direction::Down;
                } else {
                    col += 1;
                    seen.insert((row, col));
                }
            }
        }
    }
}

#[cfg(feature = "part_2")]
fn part_2(input: &'static str) -> anyhow::Result<usize> {
    let mut grid = parse_input(input)?;
    let mut loop_obstruction_count = 0;
    let Some(((row, col), Piece::Guard(direction))) = find_guard(&grid) else {
        anyhow::bail!("Where is the guard?");
    };

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let mut seen: HashSet<(usize, usize, Direction)> = HashSet::default();
            if grid[r][c] != Piece::Open {
                continue;
            }
            let original = grid[r][c].to_owned();
            grid[r][c] = Piece::Obstruction;

            // Restart on each loop
            let mut row = row;
            let mut col = col;
            let mut direction = direction;

            // start moving the guard until exit or seen
            loop {
                match direction {
                    Direction::Up => {
                        if row == 0 {
                            // guard exited, try a new one
                            break;
                        }
                        if grid[row - 1][col] == Piece::Obstruction {
                            direction = Direction::Right;
                            continue;
                        }
                        row -= 1;
                        if seen.contains(&(row, col, direction)) {
                            loop_obstruction_count += 1;
                            break;
                        }
                        seen.insert((row, col, direction));
                    }
                    Direction::Down => {
                        if row == grid.len() - 1 {
                            break;
                        }
                        if grid[row + 1][col] == Piece::Obstruction {
                            direction = Direction::Left;
                            continue;
                        }
                        row += 1;
                        if seen.contains(&(row, col, direction)) {
                            loop_obstruction_count += 1;
                            break;
                        }
                        seen.insert((row, col, direction));
                    }
                    Direction::Left => {
                        if col == 0 {
                            break;
                        }
                        if grid[row][col - 1] == Piece::Obstruction {
                            direction = Direction::Up;
                            continue;
                        }
                        col -= 1;
                        if seen.contains(&(row, col, direction)) {
                            loop_obstruction_count += 1;
                            break;
                        }
                        seen.insert((row, col, direction));
                    }
                    Direction::Right => {
                        if col == grid[row].len() - 1 {
                            break;
                        }
                        if grid[row][col + 1] == Piece::Obstruction {
                            direction = Direction::Down;
                            continue;
                        }
                        col += 1;
                        if seen.contains(&(row, col, direction)) {
                            loop_obstruction_count += 1;
                            break;
                        }
                        seen.insert((row, col, direction));
                    }
                }
            }

            grid[r][c] = original;
        }
    }

    Ok(loop_obstruction_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "part_1")]
    fn part_1_test() {
        let input = include_str!("inputs/sample.txt");
        let res = part_1(input).unwrap();
        assert_eq!(res, 41);
    }

    #[test]
    #[cfg(feature = "part_2")]
    fn part_2_test() {
        let input = include_str!("inputs/sample.txt");
        let res = part_2(input).unwrap();
        assert_eq!(res, 6);
    }
}
