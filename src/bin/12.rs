advent_of_code::solution!(12);

use std::collections::{hash_map::Entry, HashMap};

use advent_of_code::{Grid, NEIGHBOURS_ORTHOGONAL_VECTORS};

enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::parse(input, |(_y, _x), v| v);

    let mut total = 0;

    let mut checked = grid.map_collect(|(_y, _x), _v| false);
    for y in 0..grid.height {
        for x in 0..grid.width {
            let init_pos = (y, x);

            if checked[init_pos] {
                continue;
            }

            let mut checklist = vec![init_pos];
            let mut total_borders = 0;
            let mut total_area = 1;

            checked[init_pos] = true;

            let target = grid[init_pos];
            while let Some(pos) = checklist.pop() {
                let neighbours: Vec<_> = grid
                    .neighbours_orthogonal(pos)
                    .filter(|(_p, &val)| val == target)
                    .collect();
                // number of open sides
                total_borders += 4 - neighbours.len();
                for (p, _val) in neighbours {
                    if checked[p] {
                        continue;
                    }
                    total_area += 1;
                    checked[p] = true;
                    checklist.push(p);
                }
            }
            total += total_borders * total_area;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::parse(input, |(_y, _x), v| v);

    let mut total = 0;

    let mut checked = grid.map_collect(|(_y, _x), _v| false);
    for y in 0..grid.height {
        for x in 0..grid.width {
            let init_pos = (y, x);

            if checked[init_pos] {
                continue;
            }

            let mut checklist = vec![init_pos];

            // (up, down, left, right) indexed by (major axis, position along axis)
            let mut borders: [HashMap<isize, Vec<isize>>; 4] =
                core::array::from_fn(|_i| HashMap::new());

            let mut total_area = 1;

            checked[init_pos] = true;

            let target = grid[init_pos];
            while let Some(pos) = checklist.pop() {
                let (y, x) = pos;
                for (dy, dx) in NEIGHBOURS_ORTHOGONAL_VECTORS {
                    let p = (y + dy, x + dx);

                    // this represents a border for the current position
                    if !grid.is_inside(p) || grid[p] != target {
                        let (i, major, minor) = match (dy, dx) {
                            (-1, 0) => (Direction::Up as usize, y, x),
                            (1, 0) => (Direction::Down as usize, y, x),
                            (0, -1) => (Direction::Left as usize, x, y),
                            (0, 1) => (Direction::Right as usize, x, y),
                            _ => unreachable!(),
                        };

                        let values = match borders[i].entry(major) {
                            Entry::Occupied(o) => o.into_mut(),
                            Entry::Vacant(v) => v.insert(Vec::new()),
                        };

                        values.push(minor);
                        continue;
                    }

                    if checked[p] {
                        continue;
                    }

                    total_area += 1;
                    checked[p] = true;
                    checklist.push(p);
                }
            }
            let total_sides = borders
                .into_iter()
                .map(|hm| {
                    hm.into_values()
                        .map(|mut series| {
                            series.sort();
                            // test for consecutive line segments.
                            // work backwards for memory usage.
                            let mut i = series.len() - 1;
                            while i > 0 {
                                if series[i] == series[i - 1] + 1 {
                                    series.remove(i);
                                }
                                i -= 1;
                            }
                            series.len()
                        })
                        .sum::<usize>()
                })
                .sum::<usize>();
            total += total_sides * total_area;
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
