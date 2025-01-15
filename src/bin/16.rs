use std::collections::{BinaryHeap, HashMap, HashSet};

use advent_of_code::Grid;

advent_of_code::solution!(16);

const STRAIGHT_COST: usize = 1;
const TURN_COST: usize = 1000;

#[derive(PartialEq, Eq, Clone, Copy)]
struct State {
    cost: usize,
    loc: Location,
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

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct Location {
    pos: (isize, isize),
    vel: (isize, isize),
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        self.pos
            .0
            .cmp(&other.pos.0)
            .then_with(|| self.pos.1.cmp(&other.pos.1))
            .then_with(|| self.vel.0.cmp(&other.pos.0))
            .then_with(|| self.vel.1.cmp(&other.pos.1))
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct BestCostToReach {
    cost: usize,
    reached_from: Vec<Location>,
}

fn parse(input: &str) -> ((isize, isize), (isize, isize), Grid<bool>) {
    let char_grid = Grid::parse(input, |_pos, c| c);
    let walls = char_grid.map_collect(|_pos, c| *c == '#');
    let start_pos = char_grid
        .iter_tiles()
        .find_map(|(pos, c)| match c {
            'S' => Some(pos),
            _ => None,
        })
        .expect("No start char found");
    let end_pos = char_grid
        .iter_tiles()
        .find_map(|(pos, c)| match c {
            'E' => Some(pos),
            _ => None,
        })
        .expect("No end char found");
    (start_pos, end_pos, walls)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (start_pos, end_pos, walls) = parse(input);

    let mut heap = BinaryHeap::from([State {
        cost: 0,
        loc: Location {
            pos: start_pos,
            vel: (0, 1),
        },
    }]);

    let mut best_cost_to_reach = HashMap::from([(
        Location {
            pos: start_pos,
            vel: (0, 1),
        },
        0,
    )]);

    while let Some(State { cost, loc }) = heap.pop() {
        // Current state
        let Location { pos, vel } = loc;

        if pos == end_pos {
            return Some(cost);
        };

        if let Some(&past_cost) = best_cost_to_reach.get(&loc) {
            // There's a more optimal way to arrive here, cull this state.
            if cost > past_cost {
                continue;
            }
        }

        // We can only go fwd if there's no wall, but we can always turn.
        // Different possibilities for next move:

        // Go straight?
        let straight_pos = (pos.0 + vel.0, pos.1 + vel.1);
        if let Some(false) = walls.get(straight_pos) {
            let straight_loc = Location {
                pos: straight_pos,
                vel,
            };
            let straight_cost = cost + STRAIGHT_COST;
            match best_cost_to_reach.get(&straight_loc) {
                // This way is definitely inefficient, do not add to priority queue
                Some(&past_cost) if past_cost < cost => (),
                // This seems like a valid way, not more inefficient than what we've already got.
                Some(_) | None => {
                    heap.push(State {
                        cost: straight_cost,
                        loc: straight_loc,
                    });
                    best_cost_to_reach.insert(straight_loc, straight_cost);
                }
            }
        }

        // Turn left?
        let left_vel = (-vel.1, vel.0);
        let left_loc = Location { pos, vel: left_vel };
        let left_cost = cost + TURN_COST;
        match best_cost_to_reach.get(&left_loc) {
            // This way is definitely inefficient, do not add to priority queue
            Some(&past_cost) if past_cost < cost => (),
            // This seems like a valid way, not more inefficient than what we've already got.
            Some(_) | None => {
                heap.push(State {
                    cost: left_cost,
                    loc: left_loc,
                });
                best_cost_to_reach.insert(left_loc, left_cost);
            }
        }

        // Turn right?
        let right_vel = (vel.1, -vel.0);
        let right_loc = Location {
            pos,
            vel: right_vel,
        };
        let right_cost = cost + TURN_COST;
        match best_cost_to_reach.get(&right_loc) {
            // This way is definitely inefficient, do not add to priority queue
            Some(&past_cost) if past_cost < cost => (),
            // This seems like a valid way, not more inefficient than what we've already got.
            Some(_) | None => {
                heap.push(State {
                    cost: right_cost,
                    loc: right_loc,
                });
                best_cost_to_reach.insert(right_loc, right_cost);
            }
        }
    }
    // Fallback only, main return is in loop
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let (start_pos, end_pos, walls) = parse(input);

    let start_loc = Location {
        pos: start_pos,
        vel: (0, 1),
    };

    let mut heap = BinaryHeap::from([State {
        cost: 0,
        loc: start_loc,
    }]);

    let mut best_cost_to_reach = HashMap::from([(
        start_loc,
        BestCostToReach {
            cost: 0,
            reached_from: vec![start_loc],
        },
    )]);

    let mut end_state = None;

    while let Some(State { cost, loc }) = heap.pop() {
        // Current state
        let Location { pos, vel } = loc;

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
        } else if pos == end_pos {
            end_state = Some(State { cost, loc });
        };

        if let Some(past_best) = best_cost_to_reach.get(&loc) {
            // There's a more optimal way to arrive here, cull this state.
            if cost > past_best.cost {
                continue;
            }
        }

        // We can only go fwd if there's no wall, but we can always turn.
        // Different possibilities for next move:

        // Go straight?
        let straight_pos = (pos.0 + vel.0, pos.1 + vel.1);
        if let Some(false) = walls.get(straight_pos) {
            let straight_loc = Location {
                pos: straight_pos,
                vel,
            };
            let straight_cost = cost + STRAIGHT_COST;
            match best_cost_to_reach.get(&straight_loc) {
                // This solution is worse
                Some(past_best) if past_best.cost < straight_cost => (),
                // This solution is on par
                Some(past_best) if past_best.cost == straight_cost => {
                    heap.push(State {
                        cost: straight_cost,
                        loc: straight_loc,
                    });
                    let record = best_cost_to_reach
                        .get_mut(&straight_loc)
                        .expect("Key was present in HashMap a second ago");
                    record.reached_from.push(loc);
                }
                // This solution is better
                Some(_) | None => {
                    heap.push(State {
                        cost: straight_cost,
                        loc: straight_loc,
                    });
                    best_cost_to_reach.insert(
                        straight_loc,
                        BestCostToReach {
                            cost: straight_cost,
                            reached_from: vec![loc],
                        },
                    );
                }
            }
        }

        // Turn left?
        let left_vel = (-vel.1, vel.0);
        let left_loc = Location { pos, vel: left_vel };
        let left_cost = cost + TURN_COST;
        match best_cost_to_reach.get(&left_loc) {
            // This solution is worse
            Some(past_best) if past_best.cost < left_cost => (),
            // This solution is on par
            Some(past_best) if past_best.cost == left_cost => {
                heap.push(State {
                    cost: left_cost,
                    loc: left_loc,
                });
                let record = best_cost_to_reach
                    .get_mut(&left_loc)
                    .expect("Key was present in HashMap a second ago");
                record.reached_from.push(loc);
            }
            // This solution is better
            Some(_) | None => {
                heap.push(State {
                    cost: left_cost,
                    loc: left_loc,
                });
                best_cost_to_reach.insert(
                    left_loc,
                    BestCostToReach {
                        cost: left_cost,
                        reached_from: vec![loc],
                    },
                );
            }
        }

        // Turn right?
        let right_vel = (vel.1, -vel.0);
        let right_loc = Location {
            pos,
            vel: right_vel,
        };
        let right_cost = cost + TURN_COST;
        match best_cost_to_reach.get(&right_loc) {
            // This solution is worse
            Some(past_best) if past_best.cost < right_cost => (),
            // This solution is on par
            Some(past_best) if past_best.cost == right_cost => {
                heap.push(State {
                    cost: right_cost,
                    loc: right_loc,
                });
                let record = best_cost_to_reach
                    .get_mut(&right_loc)
                    .expect("Key was present in HashMap a second ago");
                record.reached_from.push(loc);
            }
            // This solution is better
            Some(_) | None => {
                heap.push(State {
                    cost: right_cost,
                    loc: right_loc,
                });
                best_cost_to_reach.insert(
                    right_loc,
                    BestCostToReach {
                        cost: right_cost,
                        reached_from: vec![loc],
                    },
                );
            }
        }
    }

    let end_state = end_state?;

    let mut queue = vec![end_state.loc];
    let mut visited = HashSet::from([end_state.loc, start_loc]);
    while let Some(loc) = queue.pop() {
        for &parent in &best_cost_to_reach[&loc].reached_from {
            // `insert` returns true on first insert
            if parent != start_loc && visited.insert(parent) {
                queue.push(parent);
            }
        }
    }

    Some(
        visited
            .iter()
            .map(|loc| loc.pos)
            .collect::<HashSet<_>>()
            .len(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_bis() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_bis() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
