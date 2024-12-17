use std::io::Write;
advent_of_code::solution!(14);

use std::{
    fs::{create_dir_all, File},
    io::BufWriter,
    path::Path,
};

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
fn simulate_pt2(input: &str, width: isize, height: isize, pathname: &str) -> &'static str {
    let routes = parse(input, width, height);
    create_dir_all(format!("{}/{}", pathname, DAY)).unwrap();

    for time in 1..=10000 {
        let positions: Vec<_> = routes.iter().map(|r| r[time % r.len()]).collect();

        if time % 1000 == 0 {
            println!("{}", time);
        }

        let name = format!(r"{}/{}/{:05}.png", pathname, DAY, time);
        let path = Path::new(&name);
        let file = File::create(path).unwrap();
        let w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, width as u32, height as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
        let source_chromaticities = png::SourceChromaticities::new(
            // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000),
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header().unwrap();

        let mut data = Vec::<u8>::new();
        for y in 0..height {
            for x in 0..width {
                data.extend(if positions.iter().any(|p| *p == (x, y)) {
                    [0, 255, 0]
                } else {
                    [0, 0, 0]
                });
            }
        }
        writer.write_image_data(&data).unwrap(); // Save        let positions: Vec<_> = routes.iter().map(|r| r[timesteps % r.len()]).collect();
    }
    "Should be OK"
}

pub fn part_two(input: &str) -> Option<&'static str> {
    Some(simulate_pt2(input, WIDTH, HEIGHT, "./imgs/solve"))
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

    #[test]
    fn test_part_two() {
        simulate_pt2(
            &advent_of_code::template::read_file("examples", DAY),
            11,
            7,
            "./imgs/test",
        );
    }
}
