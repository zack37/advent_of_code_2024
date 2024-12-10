fn main() -> anyhow::Result<()> {
    let input = include_str!("inputs/input.txt");

    #[cfg(feature = "part_1")]
    println!("Part 1: {}", part_1(input)?);

    #[cfg(feature = "part_2")]
    println!("Part 2: {}", part_2(input)?);

    Ok(())
}

#[cfg(feature = "part_1")]
fn part_1(input: &'static str) -> anyhow::Result<usize> {
    let high: u32 = input.chars().filter_map(|c| c.to_digit(10)).sum();
    let mut rev = (0..input.len())
        .rev()
        .zip(input.chars().rev())
        .scan(high, |base, (compressed_index, c)| {
            let num_indicies = c.to_digit(10).unwrap();
            *base -= num_indicies;

            Some((*base..(*base + num_indicies)).rev().filter_map(move |i| {
                (compressed_index % 2 == 0).then_some((i, compressed_index / 2))
            }))
        })
        .flatten();

    let mut base_index = 0;
    let mut sum = 0;
    let mut last_uncompressed_index = u32::MAX;

    for (compressed_index, c) in input.chars().enumerate() {
        let num_indicies = c.to_digit(10).unwrap() as usize;
        let file_id = compressed_index / 2;

        for uncompressed_index in base_index..(base_index + num_indicies) {
            if uncompressed_index >= last_uncompressed_index as usize {
                break;
            }
            if compressed_index % 2 == 0 {
                sum += uncompressed_index * file_id;
            } else {
                let (rev_uncompressed_index, file_id) = rev.next().unwrap();
                sum += uncompressed_index * file_id;
                last_uncompressed_index = rev_uncompressed_index;
            }
        }

        base_index += num_indicies;
    }

    Ok(sum)
}

#[cfg(feature = "part_2")]
struct Chunk {
    uncompressed_index: usize,
    count: usize,
    file_id: usize,
}

#[cfg(feature = "part_2")]
fn part_2(input: &'static str) -> anyhow::Result<usize> {
    let high_index: usize = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .sum();

    let uncompressed_reversed =
        (0..input.len()).rev().zip(input.chars().rev());
    let reverse = uncompressed_reversed
        .scan(
            high_index,
            |base_index, (compressed_index, c)| {
                let num_indices =
                    c.to_digit(10).unwrap() as usize;
                *base_index -= num_indices;

                if compressed_index % 2 == 0 {
                    Some(Some(Chunk {
                        uncompressed_index: *base_index,
                        count: num_indices,
                        file_id: compressed_index as usize
                            / 2,
                    }))
                } else {
                    Some(None)
                }
            },
        )
        .flatten();
    // .filter_map(|v| v);

    // (uncompressed_index, space_count)
    let mut empties = input
        .chars()
        .enumerate()
        .fold(
            (0, vec![]),
            |(mut uncompressed_index, mut empties),
             (compressed_index, c)| {
                let num_indices =
                    c.to_digit(10).unwrap() as usize;
                if compressed_index % 2 != 0 {
                    empties.push((
                        uncompressed_index,
                        num_indices,
                    ))
                }
                uncompressed_index += num_indices;
                (uncompressed_index, empties)
            },
        )
        .1;

    // let mut moved_ids: Vec<usize> = vec![];
    let mut moved_chunks: Vec<Chunk> = vec![];
    for chunk in reverse {
        let Some(empty) =
            empties.iter_mut().find(|(i, empty_space)| {
                chunk.count <= (*empty_space as usize)
                    && *i < chunk.uncompressed_index
            })
        else {
            continue;
        };

        // moved_ids.push(chunk.file_id);
        moved_chunks.push(Chunk {
            uncompressed_index: empty.0 as usize,
            ..chunk
        });
        empty.0 += chunk.count;
        empty.1 -= chunk.count;
    }

    let mut base_index = 0;
    let mut sum = 0;

    for (compressed_index, c) in input.chars().enumerate() {
        let num_indices = c.to_digit(10).unwrap() as usize;
        let file_id = compressed_index / 2;

        for uncompressed_index in
            base_index..(base_index + num_indices)
        {
            if compressed_index % 2 == 0
                && !moved_chunks.iter().any(|chunk| {
                chunk.file_id == file_id as usize
            })
            {
                sum += uncompressed_index * file_id;
            }
        }

        base_index += num_indices;
    }

    for chunk in moved_chunks.iter() {
        for index in chunk.uncompressed_index
            ..(chunk.uncompressed_index + chunk.count)
        {
            sum += index * chunk.file_id;
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "part_1")]
    fn test_part_1() {
        let input = include_str!("inputs/sample.txt");
        let answer = part_1(input).unwrap();
        assert_eq!(answer, 1928);
    }

    #[test]
    #[cfg(feature = "part_2")]
    fn test_part_2() {
        let input = include_str!("inputs/sample.txt");
        let answer = part_2(input).unwrap();
        assert_eq!(answer, 2858);
    }
}
