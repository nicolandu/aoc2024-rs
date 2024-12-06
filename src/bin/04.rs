use std::collections::HashSet;

advent_of_code::solution!(4);

type AllMap = (
    HashSet<(isize, isize)>,
    HashSet<(isize, isize)>,
    HashSet<(isize, isize)>,
    HashSet<(isize, isize)>,
);

pub fn build_all_map(input: &str) -> AllMap {
    let g: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut x = HashSet::new();
    let mut m = HashSet::new();
    let mut a = HashSet::new();
    let mut s = HashSet::new();
    for (ln, ln_str) in g.iter().enumerate() {
        for (col, c) in ln_str.iter().enumerate() {
            match c {
                'X' => &mut x,
                'M' => &mut m,
                'A' => &mut a,
                'S' => &mut s,
                _ => continue,
            }
            .insert((ln.try_into().unwrap(), col.try_into().unwrap()));
        }
    }
    (x, m, a, s)
}
pub fn part_one(input: &str) -> Option<u32> {
    let (x, m, a, s) = build_all_map(input);
    Some(
        x.iter()
            .map(|(ln, col)| {
                {
                    [
                        (-1isize, -1isize),
                        (-1, 0),
                        (-1, 1),
                        (0, 1),
                        (1, 1),
                        (1, 0),
                        (1, -1),
                        (0, -1),
                    ]
                    .iter()
                    .filter(|(dln, dcol)| {
                        [&m, &a, &s]
                            .iter()
                            .zip([1, 2, 3])
                            .all(|(set, i)| set.contains(&(ln + i * dln, col + i * dcol)))
                    })
                }
                .count()
            })
            .sum::<usize>()
            .try_into()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_x, m, a, s) = build_all_map(input);
    Some(
        a.iter()
            .filter(|(y, x)| {
                for (d1y, d1x) in [(-1, -1), (1, 1)] {
                    // diagonal 1 vector
                    for (d2y, d2x) in [(-1, 1), (1, -1)] {
                        if m.contains(&(y + d1y, x + d1x))
                            && s.contains(&(y - d1y, x - d1x))
                            && m.contains(&(y + d2y, x + d2x))
                            && s.contains(&(y - d2y, x - d2x))
                        {
                            return true;
                        }
                    }
                }
                false
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
