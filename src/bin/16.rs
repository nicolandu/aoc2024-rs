use std::collections::HashSet;

use advent_of_code::{Grid, NEIGHBOURS_ORTHOGONAL_VECTORS};
use csv::Writer;

advent_of_code::solution!(16);

/// (start_pos, end_pos, grid of walls, grid of costs, grid of dists from start)
fn parse(input: &str) -> ((isize, isize), (isize, isize), Grid<bool>) {
    let char_grid = Grid::parse(input, |_pos, c| c);
    let walls = char_grid.map_collect(|_pos, c| *c == '#');
    let start_pos = char_grid
        .iter_tiles()
        .find_map(|(pos, c)| match c {
            'S' => Some(pos),
            _ => None,
        })
        .expect("No start char found");
    let end_pos = char_grid
        .iter_tiles()
        .find_map(|(pos, c)| match c {
            'E' => Some(pos),
            _ => None,
        })
        .expect("No end char found");
    (start_pos, end_pos, walls)
}

/// Walls have a cost of u32::MAX (an arbitrarily large value).
fn calc_costs(start_pos: (isize, isize), end_pos: (isize, isize), walls: &Grid<bool>) -> Grid<u32> {
    let mut costs = walls.map_collect(|(_y, _x), _v| u32::MAX);
    // Constant number of turns
    let mut current_explo = HashSet::from([start_pos]);
    let mut explored = HashSet::from([start_pos]);

    // Start exploring east before factoring in turns
    costs[start_pos] = 0;
    let mut off = 1;
    let mut new = (start_pos.0, start_pos.1 + 1);
    while let Some(false) = walls.get(new) {
        costs[new] = off;
        off += 1;
        new.1 += 1;
        current_explo.insert(new);
        explored.insert(new);
    }

    loop {
        // Vertical moves
        let mut next_explo = HashSet::new();
        for tile in current_explo {
            // Up
            let mut off = 1;
            let mut new = (tile.0 - 1, tile.1);
            while let Some(false) = walls.get(new) {
                costs[new] = costs[new].min(costs[tile] + 1000 + off);
                off += 1;
                new.0 -= 1;
                // Explore next round if not explored before
                if explored.insert(new) {
                    next_explo.insert(new);
                }
            }
            // Down
            let mut off = 1;
            let mut new = (tile.0 + 1, tile.1);
            while let Some(false) = walls.get(new) {
                costs[new] = costs[new].min(costs[tile] + 1000 + off);
                off += 1;
                new.0 += 1;
                // Explore next round if not explored before
                if explored.insert(new) {
                    next_explo.insert(new);
                }
            }
        }
        if explored.contains(&end_pos) {
            break;
        }
        current_explo = next_explo;

        // Horizontal moves
        let mut next_explo = HashSet::new();
        for tile in current_explo {
            // Left
            let mut off = 1;
            let mut new = (tile.0, tile.1 - 1);
            while let Some(false) = walls.get(new) {
                costs[new] = costs[new].min(costs[tile] + 1000 + off);
                off += 1;
                new.1 -= 1;
                // Explore next round if not explored before
                if explored.insert(new) {
                    next_explo.insert(new);
                }
            }
            // Right
            let mut off = 1;
            let mut new = (tile.0, tile.1 + 1);
            while let Some(false) = walls.get(new) {
                costs[new] = costs[new].min(costs[tile] + 1000 + off);
                off += 1;
                new.1 += 1;
                // Explore next round if not explored before
                if explored.insert(new) {
                    next_explo.insert(new);
                }
            }
        }
        if explored.contains(&end_pos) {
            break;
        }
        current_explo = next_explo;
    }
    costs
}

pub fn part_one(input: &str) -> Option<u32> {
    let (start_pos, end_pos, walls) = parse(input);
    let costs = calc_costs(start_pos, end_pos, &walls);
    Some(costs[end_pos])
}

pub fn part_two(input: &str) -> Option<usize> {
    let (start_pos, end_pos, walls) = parse(input);
    let costs_fwd = calc_costs(start_pos, end_pos, &walls);
    let costs_rev = calc_costs(end_pos, start_pos, &walls);

    let mut queue = vec![start_pos];
    let mut explored = HashSet::from([start_pos]);

    // Walls have a cost of u32::MAX (a constant arbitrarily large value).
    while let Some(tile) = queue.pop() {
        for off in NEIGHBOURS_ORTHOGONAL_VECTORS {
            let mut mult = 1;
            let mut new = tile;

            // Bounds check for grid done here.
            while walls.get(new) == Some(&false) && mult <= 2 {
                // limit distance to hopefully prevent false positives
                new = (tile.0 + mult * off.0, tile.1 + mult * off.1);
                if explored.contains(&new) {
                    mult += 1;
                    continue;
                }
                // Check if we're moving along the correct direction in the gradient.
                if costs_fwd[new] > costs_fwd[tile] && costs_rev[new] < costs_rev[tile] {
                    if costs_fwd[new] == 3009 && walls.width == 15 {
                        dbg!(tile);
                        println!("hi");
                    }
                    explored.insert(new);
                    queue.push(new);
                }
                mult += 1;
            }
        }
    }
    let mut wtr = Writer::from_path(format!("{}.csv", walls.width)).unwrap();
    for y in 0..walls.height {
        wtr.write_record((0..walls.width).map(|x| {
            if explored.contains(&(y, x)) {
                "xxxx"
            } else {
                ""
            }
        }))
        .unwrap();
    }
    wtr.flush().unwrap();
    dbg!(&explored);

    Some(explored.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_bis() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_bis() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
