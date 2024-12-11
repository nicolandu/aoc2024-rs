advent_of_code::solution!(11);

use memoize::memoize;

#[allow(non_snake_case)]
#[memoize]
// How many stones does the value n create after num_iterations iterations?
fn count_becomes(n: u64, num_iterations: u32) -> u64 {
    if num_iterations == 0 {
        return 1;
    }
    match n {
        0 => count_becomes(1, num_iterations - 1),
        // Even number of digits: split aabb into [aa, bb].
        i if (i.ilog10() % 2) == 1 => {
            let split_factor = 10u64.pow((i.ilog10() + 1) / 2);
            count_becomes(i / split_factor, num_iterations - 1)
                + count_becomes(i % split_factor, num_iterations - 1)
        }
        i => count_becomes(i * 2024, num_iterations - 1),
    }
}

fn solve(input: &str, num_iterations: u32) -> u64 {
    input
        .trim()
        .split(' ')
        .map(|n| count_becomes(n.parse().expect("int parse error"), num_iterations))
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
