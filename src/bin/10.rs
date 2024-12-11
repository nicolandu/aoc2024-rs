use std::collections::HashSet;

advent_of_code::solution!(10);

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("digit parse error"))
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input);
    let mut heads: Vec<(usize, usize)> = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, n) in row.iter().enumerate() {
            if *n == 0 {
                heads.push((r, c));
            }
        }
    }

    Some(
        heads
            .iter()
            .map(|&head| {
                let mut current_tiles = HashSet::from([head]);
                for n in 1..=9 {
                    let mut neighbours = HashSet::new();
                    for (tile_y, tile_x) in current_tiles {
                        for (dy, dx) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                            if Some(n)
                                == grid
                                    .get(((tile_y as isize) + dy) as usize)
                                    .and_then(|row| row.get(((tile_x as isize) + dx) as usize))
                                    .copied()
                            {
                                neighbours.insert((
                                    ((tile_y as isize) + dy) as usize,
                                    ((tile_x as isize) + dx) as usize,
                                ));
                            }
                        }
                    }
                    current_tiles = neighbours;
                }
                current_tiles.len()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse(input);
    let mut heads: Vec<(usize, usize)> = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, n) in row.iter().enumerate() {
            if *n == 0 {
                heads.push((r, c));
            }
        }
    }

    // F*** it, we don't care about no time complexity here
    Some(
        heads
            .iter()
            .map(|&head| {
                let mut current_trails = HashSet::from([vec![head]]);
                for n in 1..=9 {
                    let mut with_neighbours = HashSet::new();
                    for trail in current_trails {
                        let (tile_y, tile_x) = trail.last().unwrap().to_owned();
                        for (dy, dx) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                            if Some(n)
                                == grid
                                    .get(((tile_y as isize) + dy) as usize)
                                    .and_then(|row| row.get(((tile_x as isize) + dx) as usize))
                                    .copied()
                            {
                                with_neighbours.insert({
                                    let mut c = trail.clone();
                                    c.push((
                                        ((tile_y as isize) + dy) as usize,
                                        ((tile_x as isize) + dx) as usize,
                                    ));
                                    c
                                });
                            }
                        }
                    }
                    current_trails = with_neighbours;
                }
                current_trails.len()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
