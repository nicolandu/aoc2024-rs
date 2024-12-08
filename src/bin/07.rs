advent_of_code::solution!(7);

#[derive(Debug)]
struct Equation {
    expected: u64,
    vals: Vec<u64>,
}

fn parse(input: &str) -> impl use<'_> + Iterator<Item = Equation> {
    input.lines().map(|l| {
        let (expected, vals) = l.split_once(": ").expect("should have a \": \"");
        let expected = expected.parse().expect("parse error");
        let vals = vals
            .split(' ')
            .map(|v| v.parse().expect("parse error"))
            .collect();
        Equation { expected, vals }
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .filter_map(|eq| {
                let num_ops = eq.vals.len() - 1;
                for ops in 0u32..(1 << num_ops) {
                    let mut it = eq.vals.iter();
                    let mut acc = *it.next().expect("no empty equation please");
                    for (i, val) in it.enumerate() {
                        match (ops >> i) & 1 {
                            0 => acc += val,
                            1 => acc *= val,
                            _ => unreachable!(),
                        }
                    }
                    if acc == eq.expected {
                        return Some(eq.expected);
                    }
                }
                None
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .filter_map(|eq| {
                let num_ops = eq.vals.len() - 1;
                for mut ops in 0u32..(3u32.pow(num_ops.try_into().unwrap())) {
                    let mut it = eq.vals.iter();
                    let mut acc = *it.next().expect("no empty equation please");
                    for val in it {
                        match ops % 3 {
                            0 => acc += val,
                            1 => acc *= val,
                            2 => acc = acc * 10u64.pow(val.checked_ilog10().unwrap_or(0) + 1) + val,
                            _ => unreachable!(),
                        }
                        ops /= 3;
                    }
                    if acc == eq.expected {
                        return Some(eq.expected);
                    }
                }
                None
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
