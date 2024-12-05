fn main() -> anyhow::Result<()> {
    let input = include_str!("inputs/input.txt");
    #[cfg(feature = "part_1")]
    {
        let answer = part_1(input)?;
        println!("Part 1: {:?}", answer);
    }
    #[cfg(feature = "part_2")]
    {
        let answer = part_2(input)?;
        println!("Part 2: {:?}", answer);
    }

    Ok(())
}

fn parse(input: &'static str) -> anyhow::Result<Vec<Vec<char>>> {
    let result = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Ok(result)
}

#[cfg(feature = "part_1")]
fn part_1(input: &'static str) -> anyhow::Result<i32> {
    let grid = parse(input)?;

    let mut count = 0;
    let row_len = grid.len();
    let col_len = grid[0].len();
    for row in 0..row_len {
        for col in 0..col_len {
            if grid[row][col] != 'X' {
                continue;
            }

            let row_ptr = row as i32;
            let col_ptr = col as i32;
            let row_len = row_len as i32;
            let col_len = col_len as i32;

            // Check N
            if row_ptr - 3 >= 0
                && grid[row - 1][col] == 'M'
                && grid[row - 2][col] == 'A'
                && grid[row - 3][col] == 'S'
            {
                count += 1;
            }
            // Check NE
            if row_ptr - 3 >= 0
                && col_ptr + 3 < col_len
                && grid[row - 1][col + 1] == 'M'
                && grid[row - 2][col + 2] == 'A'
                && grid[row - 3][col + 3] == 'S'
            {
                count += 1;
            }
            // Check E
            if col_ptr + 3 < col_len
                && grid[row][col + 1] == 'M'
                && grid[row][col + 2] == 'A'
                && grid[row][col + 3] == 'S'
            {
                count += 1;
            }
            // Check SE
            if row_ptr + 3 < row_len
                && col_ptr + 3 < col_len
                && grid[row + 1][col + 1] == 'M'
                && grid[row + 2][col + 2] == 'A'
                && grid[row + 3][col + 3] == 'S'
            {
                count += 1;
            }
            // Check S
            if row_ptr + 3 < row_len
                && grid[row + 1][col] == 'M'
                && grid[row + 2][col] == 'A'
                && grid[row + 3][col] == 'S'
            {
                count += 1;
            }
            // Check SW
            if row_ptr + 3 < row_len
                && col_ptr - 3 >= 0
                && grid[row + 1][col - 1] == 'M'
                && grid[row + 2][col - 2] == 'A'
                && grid[row + 3][col - 3] == 'S'
            {
                count += 1;
            }
            // Check W
            if col_ptr - 3 >= 0
                && grid[row][col - 1] == 'M'
                && grid[row][col - 2] == 'A'
                && grid[row][col - 3] == 'S'
            {
                count += 1;
            }
            // Check NW
            if row_ptr - 3 >= 0
                && col_ptr - 3 >= 0
                && grid[row - 1][col - 1] == 'M'
                && grid[row - 2][col - 2] == 'A'
                && grid[row - 3][col - 3] == 'S'
            {
                count += 1;
            }
        }
    }

    Ok(count)
}
#[cfg(feature = "part_2")]
fn part_2(input: &'static str) -> anyhow::Result<i32> {
    let grid = parse(input)?;

    let mut count = 0;
    let row_len = grid.len();
    let col_len = grid[0].len();
    for row in 0..row_len {
        for col in 0..col_len {
            if grid[row][col] != 'A' {
                continue;
            }

            let row_ptr = row as i32;
            let col_ptr = col as i32;
            let row_len = row_len as i32;
            let col_len = col_len as i32;

            if row_ptr - 1 < 0
                || row_ptr + 1 >= row_len
                || col_ptr - 1 < 0
                || col_ptr + 1 >= col_len
            {
                continue;
            }

            if (grid[row - 1][col - 1] == 'M' && grid[row + 1][col + 1] == 'S'
                || grid[row - 1][col - 1] == 'S' && grid[row + 1][col + 1] == 'M')
                && (grid[row - 1][col + 1] == 'M' && grid[row + 1][col - 1] == 'S'
                    || grid[row - 1][col + 1] == 'S' && grid[row + 1][col - 1] == 'M')
            {
                count += 1;
            }
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = include_str!("inputs/sample.txt");
        assert_eq!(part_1(input).unwrap(), 18);
    }

    #[test]
    fn part_2_test() {
        let input = include_str!("inputs/sample.txt");
        assert_eq!(part_2(input).unwrap(), 9);
    }
}
