use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

struct AntennaMap {
    height: isize,
    width: isize,
    antennas: HashMap<char, HashSet<(isize, isize)>>,
}

fn parse(input: &str) -> AntennaMap {
    let matrix: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut m = AntennaMap {
        height: matrix.len().try_into().unwrap(),
        width: matrix[0].len().try_into().unwrap(),
        antennas: HashMap::new(),
    };
    for (y, row) in matrix.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if let 'a'..='z' | 'A'..='Z' | '0'..='9' = c {
                if !m.antennas.contains_key(c) {
                    m.antennas.insert(*c, HashSet::new());
                }
                m.antennas
                    .get_mut(c)
                    .unwrap()
                    .insert((y.try_into().unwrap(), x.try_into().unwrap()));
            }
        }
    }
    m
}

pub fn part_one(input: &str) -> Option<u32> {
    let m = parse(input);
    let mut cnt: u32 = 0;
    for y in 0..m.height {
        'search: for x in 0..m.width {
            for (_label, pos_set) in m.antennas.iter() {
                for (ant_y, ant_x) in pos_set.iter() {
                    let dy = ant_y - y;
                    let dx = ant_x - x;
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    if pos_set.contains(&(y + 2 * dy, x + 2 * dx)) {
                        cnt += 1;
                        continue 'search;
                    }
                }
            }
        }
    }
    Some(cnt)
}

pub fn part_two(input: &str) -> Option<u32> {
    let m = parse(input);
    let mut cnt: u32 = 0;
    for y1 in 0..m.height {
        'search: for x1 in 0..m.width {
            for (_label, pos_set) in m.antennas.iter() {
                for (y2, x2) in pos_set.iter() {
                    for (y3, x3) in pos_set.iter() {
                        if y2 == y3 && x2 == x3 {
                            continue;
                        }
                        // collinearity of 3 points function
                        if x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2) == 0 {
                            cnt += 1;
                            continue 'search;
                        }
                    }
                }
            }
        }
    }
    Some(cnt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
