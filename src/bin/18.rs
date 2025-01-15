use advent_of_code::{Grid, NEIGHBOURS_ORTHOGONAL_VECTORS};
use std::collections::{BinaryHeap, HashMap};
advent_of_code::solution!(18);

#[derive(PartialEq, Eq, Clone, Copy)]
struct State {
    cost: usize,
    loc: (isize, isize),
}

// Code from the standard library.
// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.loc.cmp(&other.loc))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_inner(input, 71, 71, 1024)
}

pub fn part_two(input: &str) -> Option<&str> {
    part_two_inner(input, 71, 71)
}

#[inline]
/// Takes height and width, not max Y and max X.
fn part_one_inner(input: &str, height: isize, width: isize, restriction: usize) -> Option<usize> {
    let mut appearances = Grid::new(height, width, |_| 0usize);
    for (i, l) in input.lines().enumerate() {
        let (x, y) = l.split_once(",").expect("Parse error");
        let x = x.parse().expect("Parse error");
        let y = y.parse().expect("Parse error");
        *appearances
            .get_mut((y, x))
            .expect("Tile index out of range when parsing") = i + 1;
    }
    find_cost(&appearances, restriction)
}

#[inline]
/// Takes height and width, not max Y and max X.
fn part_two_inner(input: &str, height: isize, width: isize) -> Option<&str> {
    let mut appearances = Grid::new(height, width, |_| 0usize);
    for (i, l) in input.lines().enumerate() {
        let (x, y) = l.split_once(",").expect("Parse error");
        let x = x.parse().expect("Parse error");
        let y = y.parse().expect("Parse error");
        *appearances
            .get_mut((y, x))
            .expect("Tile index out of range when parsing") = i + 1;
    }
    (0..(input.lines().count()))
        .map(|i| (i, find_cost(&appearances, i + 1)))
        // Find first block causing a problem
        .find_map(|(i, cost)| match cost {
            Some(_c) => None,
            None => Some(i),
        })
        .and_then(|i| input.lines().nth(i))
}

/// Takes height and width, not max Y and max X.
fn find_cost(appearances: &Grid<usize>, restriction: usize) -> Option<usize> {
    let height = appearances.height;
    let width = appearances.width;
    let start_loc = (0, 0);
    let end_pos = (height - 1, width - 1);

    let mut heap = BinaryHeap::from([State {
        cost: 0,
        loc: start_loc,
    }]);

    let mut best_cost_to_reach = HashMap::from([(start_loc, 0)]);

    let mut end_state = None;

    while let Some(State { cost, loc }) = heap.pop() {
        // Instead of short-circuiting if we reach the end_pos, we keep track of its cost.
        if let Some(State {
            cost: best_cost,
            loc: _loc,
        }) = end_state
        {
            // Not a viable solution
            if cost > best_cost {
                continue;
            }
        } else if loc == end_pos {
            end_state = Some(State { cost, loc });
        };

        if let Some(&past_best) = best_cost_to_reach.get(&loc) {
            // There's a more optimal way to arrive here, cull this state.
            if cost > past_best {
                continue;
            }
        }

        for off in NEIGHBOURS_ORTHOGONAL_VECTORS {
            let new_loc = (loc.0 + off.0, loc.1 + off.1);
            if let Some(&t) = appearances.get(new_loc) {
                // Cannot move to tile
                if t != 0 && t <= restriction {
                    continue;
                }
                let new_cost = cost + 1;
                match best_cost_to_reach.get(&new_loc) {
                    // This solution is worse/equal
                    Some(&past_best) if past_best <= new_cost => (),
                    // This solution is better
                    Some(_) | None => {
                        heap.push(State {
                            cost: new_cost,
                            loc: new_loc,
                        });
                        best_cost_to_reach.insert(new_loc, new_cost);
                    }
                }
            }
        }
    }

    let end_state = end_state?;

    Some(end_state.cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_inner(
            &advent_of_code::template::read_file("examples", DAY),
            7,
            7,
            12,
        );
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let test_case = advent_of_code::template::read_file("examples", DAY);
        let result = part_two_inner(&test_case, 7, 7);
        assert_eq!(result, Some("6,1"));
    }
}
