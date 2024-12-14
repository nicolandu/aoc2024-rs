use eqsolver::multivariable::MultiVarNewtonFD;
use nalgebra::{vector, Vector2};

advent_of_code::solution!(13);

struct Prize {
    a: Vector2<f64>,
    b: Vector2<f64>,
    tgt: Vector2<f64>,
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
            a: Vector2::new(ax, ay),
            b: Vector2::new(bx, by),
            tgt: Vector2::new(tx, ty),
        })
    })
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        parse(input)
            .filter_map(|p| {
                let f = |v: Vector2<f64>| p.tgt - p.a * v[0] - p.b * v[1];
                let Ok(sol) = MultiVarNewtonFD::new(f)
                    .with_itermax(200)
                    .solve(vector![100., 100.])
                else {
                    return None;
                };

                let (a, b) = (sol[0].round(), sol[1].round());
                // we're off, not a good solution
                if f(vector![a, b]).norm() > 1e-3 {
                    return None;
                };

                Some(a as i64 * 3 + b as i64)
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
                tgt: p.tgt,
            })
            .filter_map(|p| {
                // try to get in general diagonal direction, as differences in x and y are minor
                // compared to their value
                let f = |v: Vector2<f64>| {
                    let guess = p.a * v[0] + p.b * v[1];
                    let target = vector![1., 1.,];
                    target - guess
                };

                // how much of a and b in a [1,1] vector?
                let res = MultiVarNewtonFD::new(f)
                    .with_itermax(2000)
                    .with_tol(1e-10)
                    .solve(vector![0.1, 0.1])
                    .expect("solve error");

                // how much of a and b to get close to answer?
                let (a_init, b_init) = (
                    (res[0] * (1e13 - 10000.)).round() as i64,
                    (res[1] * (1e13 - 10000.)).round() as i64,
                );

                let err_x = 10000000000000 - a_init * (p.a[0] as i64) - b_init * (p.b[0] as i64);
                let err_y = 10000000000000 - a_init * (p.a[1] as i64) - b_init * (p.b[1] as i64);
                let tgt = p.tgt + Vector2::new(err_x as f64, err_y as f64);

                let g = |v: Vector2<f64>| tgt - p.a * v[0] - p.b * v[1];
                let res = MultiVarNewtonFD::new(g)
                    .with_itermax(200)
                    .solve(vector![100., 100.]);
                let Ok(sol) = res else {
                    dbg!(&res);
                    return None;
                };

                let (a, b) = (sol[0].round(), sol[1].round());
                // we're off, not a good solution
                if g(vector![a, b]).norm() > 1e-3 {
                    return None;
                };

                Some((a as i64 + a_init) * 3 + (b as i64 + b_init))
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
