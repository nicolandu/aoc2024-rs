use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(6);

/* COORD SYSTEM: (row, col) */
#[derive(Debug, Clone)]
struct Map {
    obstructed: Vec<Vec<bool>>,
    start: (isize, isize),
    start_orientation: (isize, isize), // unit vector
    width: isize,
    height: isize,
}

impl Map {
    fn inside(&self, pos: (isize, isize)) -> bool {
        0 <= pos.0 && pos.0 < self.height && 0 <= pos.1 && pos.1 < self.width
    }
    fn collides(&self, pos: (isize, isize)) -> bool {
        if !self.inside(pos) {
            return false;
        }
        self.obstructed[pos.0 as usize][pos.1 as usize]
    }
}

fn parse_map(input: &str) -> Map {
    let mut start = (0isize, 0isize);
    let obstructed: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == '^' {
                        start = (row.try_into().unwrap(), col.try_into().unwrap());
                    }

                    c == '#'
                })
                .collect()
        })
        .collect();
    Map {
        start_orientation: (-1, 0),
        start,
        width: obstructed[0].len().try_into().unwrap(),
        height: obstructed.len().try_into().unwrap(),
        obstructed,
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_map(input);

    let mut pos = map.start;

    let mut ori = map.start_orientation;

    let mut visited = HashSet::new();
    loop {
        let mut new = (pos.0 + ori.0, pos.1 + ori.1);
        while map.collides(new) {
            ori = (ori.1, -ori.0); // rotate (y,x) by 90deg right

            new = (pos.0 + ori.0, pos.1 + ori.1);
        }
        visited.insert(pos);
        if !map.inside(new) {
            break;
        }
        pos = new;
    }

    Some(visited.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_map(input);

    let mut pos = map.start;

    let mut ori = map.start_orientation;

    let mut visited = HashSet::new();
    loop {
        let mut new = (pos.0 + ori.0, pos.1 + ori.1);
        while map.collides(new) {
            ori = (ori.1, -ori.0); // rotate (y,x) by 90deg right

            new = (pos.0 + ori.0, pos.1 + ori.1);
        }
        if !map.inside(new) {
            break;
        }
        visited.insert((new, ori));
        pos = new;
    }

    Some(
        visited
            .clone()
            .into_iter()
            .unique_by(|(pos, _ori)| *pos)
            .filter(|(new_pos, _new_ori)| {
                // block a position on the path
                let mut now_map = map.clone();
                now_map.obstructed[new_pos.0 as usize][new_pos.1 as usize] = true;
                let map = now_map;

                // seems difficult to check where we're gonna go simply from new_pos and associated
                // orientation. Simply simulate from start.
                let mut pos = map.start;

                let mut ori = map.start_orientation;

                let mut visited = HashSet::new();
                loop {
                    let mut new = (pos.0 + ori.0, pos.1 + ori.1);
                    while map.collides(new) {
                        ori = (ori.1, -ori.0); // rotate (y,x) by 90deg right

                        new = (pos.0 + ori.0, pos.1 + ori.1);
                    }
                    // gone outside: no loop
                    if !map.inside(new) {
                        return false;
                    }
                    // returns false if already in set (same position and orientation), so it's a
                    // win
                    if !visited.insert((new, ori)) {
                        return true;
                    };
                    pos = new;
                }
            })
            .count()
            .try_into()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
