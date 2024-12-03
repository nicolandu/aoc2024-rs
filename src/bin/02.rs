advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|i| i.parse().unwrap()).collect())
        .collect();
    Some(
        reports
            .into_iter()
            .filter(|r| {
                r.windows(2).all(|w| (1..=3).contains(&(w[1] - w[0])))
                    || r.windows(2).all(|w| (-3..=-1).contains(&(w[1] - w[0])))
            })
            .count()
            .try_into()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|i| i.parse().unwrap()).collect())
        .collect();
    Some(
        reports
            .into_iter()
            .filter(|r| {
                // removing an element does not limit possibilities, so no edge case by always
                (0..r.len())
                    .map(|n| {
                        r.iter()
                            .enumerate()
                            .filter_map(move |(i, v)| if i != n { Some(v) } else { None })
                    })
                    .map(|rep| {
                        let r = rep.collect::<Vec<_>>();
                        r.windows(2).all(|w| (1..=3).contains(&(w[1] - w[0])))
                            || r.windows(2).all(|w| (-3..=-1).contains(&(w[1] - w[0])))
                    })
                    .any(|x| x)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
