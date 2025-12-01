advent_of_code::solution!(19);

/// Returns (towels, pats)
fn parse(input: &str) -> (Vec<&[u8]>, Vec<&[u8]>) {
    let (towels, pats) = input.split_once("\n\n").expect("No blank line found");
    let towels = towels.split(", ").map(|s| s.as_bytes()).collect();
    let pats = pats
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes())
        .collect();
    (towels, pats)
}

fn count_ways(towels: &[&[u8]], pat: &[u8]) -> usize {
    let mut cnt = vec![0; pat.len() + 1];
    cnt[0] = 1;

    // For each tile, count ways to reach it considering the number of ways prior.
    for i in 1..=pat.len() {
        for &towel in towels {
            if towel.len() <= i && *towel == pat[i - towel.len()..i] {
                cnt[i] += cnt[i - towel.len()];
            }
        }
    }
    cnt[pat.len()]
}

pub fn part_one(input: &str) -> Option<usize> {
    let (towels, pats) = parse(input);

    Some(
        pats.iter()
            .filter(|pat| count_ways(&towels, pat) > 0)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (towels, pats) = parse(input);

    Some(pats.iter().map(|pat| count_ways(&towels, pat)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
