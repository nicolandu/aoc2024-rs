advent_of_code::solution!(13);

struct Prize {
    a: (i64, i64),
    b: (i64, i64),
    tgt: (i64, i64),
}

// no vectors have either x=0 nor y=0.
fn parse(input: &str) -> impl Iterator<Item = Prize> + use<'_> {
    input.split("\n\n").filter_map(|p| {
        let mut l = p.lines();
        let a = l.next()?;
        let b = l.next()?;
        let t = l.next()?;

        let a = a.strip_prefix("Button A: X+").unwrap();
        let b = b.strip_prefix("Button B: X+").unwrap();
        let t = t.strip_prefix("Prize: X=").unwrap();

        let (ax, ay) = a.split_once(", Y+").unwrap();
        let (bx, by) = b.split_once(", Y+").unwrap();
        let (tx, ty) = t.split_once(", Y=").unwrap();

        let ax = ax.parse().unwrap();
        let ay = ay.parse().unwrap();
        let bx = bx.parse().unwrap();
        let by = by.parse().unwrap();
        let tx = tx.parse().unwrap();
        let ty = ty.parse().unwrap();

        Some(Prize {
            a: (ax, ay),
            b: (bx, by),
            tgt: (tx, ty),
        })
    })
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        parse(input)
            .filter_map(|p| {
                let (ax, ay) = p.a;
                let (bx, by) = p.b;
                let (tx, ty) = p.tgt;
                // thanks to WolframAlpha:
                // https://www.wolframalpha.com/input?i2d=true&i=a%7B%7BSubscript%5Ba%2Cx%5D%7D%2C%7BSubscript%5Ba%2Cy%5D%7D%7D%2Bb%7B%7BSubscript%5Bb%2Cx%5D%7D%2C%7BSubscript%5Bb%2Cy%5D%7D%7D%3D%7B%7BSubscript%5Bt%2Cx%5D%7D%2C%7BSubscript%5Bt%2Cy%5D%7D%7D+solve+for+%5C%2840%29a%5C%2844%29b%5C%2841%29
                let a = (by * tx - bx * ty) / (ax * by - ay * bx);
                let b = (ay * tx - ax * ty) / (ay * bx - ax * by);
                // check if solution works
                if a * ax + b * bx == tx && a * ay + b * by == ty {
                    Some(a * 3 + b)
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(
        parse(input)
            .map(|p| Prize {
                a: p.a,
                b: p.b,
                tgt: (p.tgt.0 + 10000000000000, p.tgt.1 + 10000000000000),
            })
            .filter_map(|p| {
                let (ax, ay) = p.a;
                let (bx, by) = p.b;
                let (tx, ty) = p.tgt;
                // thanks to WolframAlpha:
                // https://www.wolframalpha.com/input?i2d=true&i=a%7B%7BSubscript%5Ba%2Cx%5D%7D%2C%7BSubscript%5Ba%2Cy%5D%7D%7D%2Bb%7B%7BSubscript%5Bb%2Cx%5D%7D%2C%7BSubscript%5Bb%2Cy%5D%7D%7D%3D%7B%7BSubscript%5Bt%2Cx%5D%7D%2C%7BSubscript%5Bt%2Cy%5D%7D%7D+solve+for+%5C%2840%29a%5C%2844%29b%5C%2841%29
                let a = (by * tx - bx * ty) / (ax * by - ay * bx);
                let b = (ay * tx - ax * ty) / (ay * bx - ax * by);
                // check if solution works
                if a * ax + b * bx == tx && a * ay + b * by == ty {
                    Some(a * 3 + b)
                } else {
                    None
                }
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
        assert_eq!(result, Some(480));
    }
}
