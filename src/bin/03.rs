advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("failed regex");
    Some(
        re.captures_iter(input)
            .map(|caps| {
                let (_full, [a, b]) = caps.extract();
                a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(?<main>do\(\)|don't\(\)|mul\((?<a>\d{1,3}),(?<b>\d{1,3})\))")
        .expect("failed regex");
    let mut active = true;
    let mut cnt = 0;

    for cap in re.captures_iter(input) {
        match &cap["main"] {
            "do()" => active = true,
            "don't()" => active = false,
            _ if active => {
                cnt += cap["a"].parse::<u32>().unwrap() * cap["b"].parse::<u32>().unwrap()
            }

            _ => (),
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
