advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut a, mut b): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut it = l.split_whitespace();
            (
                it.next().unwrap().parse::<u32>().unwrap(),
                it.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .unzip();
    a.sort();
    b.sort();
    Some(a.into_iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (a, b): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut it = l.split_whitespace();
            (
                it.next().unwrap().parse::<u32>().unwrap(),
                it.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .unzip();
    Some(
        a.into_iter()
            .map(|a| a * u32::try_from(b.iter().filter(|&e| *e == a).count()).unwrap())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
