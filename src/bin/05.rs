use std::cmp::Ordering;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (pats, tests) = input.split_once("\n\n").expect("no empty line!");
    let pats: Vec<(u32, u32)> = pats
        .lines()
        .map(|l| l.split_once('|').expect("no pipe in line"))
        .map(|(a, b)| {
            (
                a.parse().expect("error parsing int"),
                b.parse().expect("error parsing int"),
            )
        })
        .collect();
    let tests = tests.lines().map(|l| {
        l.split(',')
            .map(|n| n.parse().expect("error parsing int"))
            .collect::<Vec<u32>>()
    });
    Some(
        tests
            .filter(|l| {
                pats.iter().all(|(a, b)| {
                    let ia = l.iter().position(|e| e == a);
                    let ib = l.iter().position(|e| e == b);
                    match (ia, ib) {
                        (Some(ia), Some(ib)) => ia < ib,
                        _ => true,
                    }
                })
            })
            .map(|l| l[l.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (pats, tests) = input.split_once("\n\n").expect("no empty line!");
    let pats: Vec<(u32, u32)> = pats
        .lines()
        .map(|l| l.split_once('|').expect("no pipe in line"))
        .map(|(a, b)| {
            (
                a.parse().expect("error parsing int"),
                b.parse().expect("error parsing int"),
            )
        })
        .collect();
    let tests = tests.lines().map(|l| {
        l.split(',')
            .map(|n| n.parse().expect("error parsing int"))
            .collect::<Vec<u32>>()
    });
    Some(
        tests
            .filter(|l| {
                // those that don't work only
                !pats.iter().all(|(a, b)| {
                    let ia = l.iter().position(|e| e == a);
                    let ib = l.iter().position(|e| e == b);
                    match (ia, ib) {
                        (Some(ia), Some(ib)) => ia < ib,
                        _ => true,
                    }
                })
            })
            .map(|mut l| {
                l.sort_by(|&a, &b| {
                    // if reverse order is explicitly enforced, swap, else do nothing
                    if pats.contains(&(b, a)) {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                });
                l
            })
            .map(|l| l[l.len() / 2])
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
