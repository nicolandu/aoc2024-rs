advent_of_code::solution!(14);

use regex::Regex;

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

// X,Y here
fn parse(input: &str, width: isize, height: isize) -> Vec<Vec<(isize, isize)>> {
    let re = Regex::new(r"(?m)^p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)$")
        .expect("failed regex");

    re.captures_iter(input)
        .map(|caps| {
            let start: (isize, isize) = (
                caps.name("px")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("parse error"),
                caps.name("py")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("parse error"),
            );
            let vel: (isize, isize) = (
                caps.name("vx")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("parse error"),
                caps.name("vy")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("parse error"),
            );

            let mut cycle = vec![start];
            let mut pos = start;

            loop {
                pos.0 += vel.0;
                // negative number shenanigans
                pos.0 = pos.0.rem_euclid(width);
                pos.1 += vel.1;
                // negative number shenanigans
                pos.1 = pos.1.rem_euclid(height);

                if pos == start {
                    break;
                };
                cycle.push(pos);
            }
            cycle
        })
        .collect()
}

// Allows to decouple width and height for the test harness.
fn simulate_pt1(input: &str, width: isize, height: isize, timesteps: usize) -> usize {
    let routes = parse(input, width, height);
    let positions: Vec<_> = routes.iter().map(|r| r[timesteps % r.len()]).collect();
    [
        ((0..width / 2), (0..height / 2)),
        ((width / 2 + 1..width), (0..height / 2)),
        ((0..width / 2), (height / 2 + 1..height)),
        ((width / 2 + 1..width), (height / 2 + 1..height)),
    ]
    .iter()
    .map(|(rx, ry)| {
        positions
            .iter()
            .filter(|(px, py)| rx.contains(px) && ry.contains(py))
            .count()
    })
    .product()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(simulate_pt1(input, WIDTH, HEIGHT, 100))
}

// We reverse engineer the initial setup by which the problem is generated and in which no tiles
// overlap.
fn simulate_pt2(input: &str, width: isize, height: isize) -> Option<usize> {
    let routes = parse(input, width, height);

    for time in 1..=10000 {
        let mut positions: Vec<_> = routes.iter().map(|r| r[time % r.len()]).collect();

        positions.sort();
        // Make sure all are different
        if positions
            .windows(2)
            .all(|s| s[0].0 != s[1].0 || s[0].1 != s[1].1)
        {
            return Some(time);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    simulate_pt2(input, WIDTH, HEIGHT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = simulate_pt1(
            &advent_of_code::template::read_file("examples", DAY),
            11,
            7,
            100,
        );
        assert_eq!(result, 12);
    }
}
