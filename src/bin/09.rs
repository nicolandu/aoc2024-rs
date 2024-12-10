use itertools::Itertools;

advent_of_code::solution!(9);

#[derive(Clone, Copy, Eq, PartialEq)]
enum Block1 {
    Empty,
    Data { id: usize },
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Block2Variant {
    Empty,
    Data { id: usize },
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct BlockData2 {
    variant: Block2Variant,
    sz: usize,
}

fn parse1(input: &str) -> Vec<Block1> {
    let mut disk = Vec::new();
    for (cur_id, mut chunk) in input
        .strip_suffix('\n')
        .unwrap_or(input)
        .chars()
        .chunks(2)
        .into_iter()
        .enumerate()
    {
        let dlen = chunk
            .next()
            .unwrap()
            .to_digit(10)
            .expect("parse error")
            .try_into()
            .expect("u32 to usize error");
        disk.extend((0..dlen).map(|_i| Block1::Data { id: cur_id }));

        let Some(elen) = chunk.next() else {
            break;
        };
        let elen = elen
            .to_digit(10)
            .expect("parse error")
            .try_into()
            .expect("u32 to usize error");
        disk.extend((0..elen).map(|_i| Block1::Empty));
    }
    assert!(!disk.is_empty(), "disk is empty!");
    disk
}

fn parse2(input: &str) -> Vec<BlockData2> {
    let mut disk = Vec::new();
    for (cur_id, mut chunk) in input
        .strip_suffix('\n')
        .unwrap_or(input)
        .chars()
        .chunks(2)
        .into_iter()
        .enumerate()
    {
        let dlen = chunk
            .next()
            .unwrap()
            .to_digit(10)
            .expect("parse error")
            .try_into()
            .expect("u32 to usize error");
        disk.push(BlockData2 {
            variant: Block2Variant::Data { id: cur_id },
            sz: dlen,
        });

        let Some(elen) = chunk.next() else {
            break;
        };
        let elen = elen
            .to_digit(10)
            .expect("parse error")
            .try_into()
            .expect("u32 to usize error");
        disk.push(BlockData2 {
            variant: Block2Variant::Empty,
            sz: elen,
        });
    }
    assert!(!disk.is_empty(), "disk is empty!");
    disk
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut disk = parse1(input);
    let mut r_index = disk.len() - 1;
    let mut w_index = 0;

    // Allowed for legibility within the loop.
    #[allow(clippy::while_let_loop)]
    loop {
        // Find the first empty position to write to.
        if let Some(i) = disk[w_index..]
            .iter()
            .position(|block| matches!(block, Block1::Empty))
        // as the index returned by position() is within the iterator
        {
            w_index += i;
        } else {
            break;
        };
        // Find the last non-empty position to read from.
        if let Some(i) = disk[..=r_index]
            .iter()
            .rev()
            .position(|block| matches!(block, Block1::Data { id: _ }))
        // as the index returned by position() is within the iterator
        {
            r_index -= i;
        } else {
            break;
        };

        // Check if we met in the middle.
        if w_index >= r_index {
            break;
        }

        // We know the two indices aren't aliased to each other.
        disk[w_index] = disk[r_index];
        disk[r_index] = Block1::Empty;
    }

    Some(
        disk.iter()
            .enumerate()
            .map_while(|(i, block)| {
                let Block1::Data { id } = block else {
                    return None;
                };
                Some(i * id)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut disk = parse2(input);

    // Inefficient, but *whatever*.
    let cur_read_id = disk
        .iter()
        .filter_map(|block_data| match block_data.variant {
            Block2Variant::Empty => None,
            Block2Variant::Data { id } => Some(id),
        })
        .max()
        .unwrap();

    for read_id in (0..=cur_read_id).rev() {
        let (read_pos, read_val) = disk
            .iter()
            .find_position(|block_data| block_data.variant == Block2Variant::Data { id: read_id })
            .unwrap();

        let read_val = *read_val;

        // Check if we can move the data to the left.
        let Some((write_pos, cur_space)) = disk[0..read_pos].iter().find_position(|block_data| {
            block_data.variant == Block2Variant::Empty && block_data.sz >= read_val.sz
        }) else {
            continue;
        };

        let left_unused = cur_space.sz - read_val.sz;
        disk[write_pos] = read_val;
        disk[read_pos] = BlockData2 {
            variant: Block2Variant::Empty,
            sz: read_val.sz,
        };

        if left_unused > 0 {
            disk.insert(
                write_pos + 1,
                BlockData2 {
                    variant: Block2Variant::Empty,
                    sz: left_unused,
                },
            );
        }
    }

    let mut checksum = 0;
    let mut i = 0;
    for block_data in disk {
        match block_data {
            BlockData2 {
                variant: Block2Variant::Empty,
                sz,
            } => i += sz,
            BlockData2 {
                variant: Block2Variant::Data { id },
                sz,
            } => {
                // Sum from one number to another (kudos to https://github.com/MaximeFagnan for
                // pointing it out after the fact, my code was much uglier!).
                checksum += id * sz * (2 * i + sz - 1) / 2;
                i += sz;
            }
        }
    }
    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
