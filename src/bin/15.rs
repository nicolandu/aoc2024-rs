advent_of_code::solution!(15);

use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;

use advent_of_code::Grid;

#[derive(Eq, PartialEq)]
enum Tile {
    Empty,
    Box,
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => '.',
                Self::Box => 'O',
                Self::Wall => '#',
            }
        )
    }
}

/// Returns (bot_y, bot_x), Grid<Tile>
fn parse_grid(input: &str) -> ((isize, isize), Grid<Tile>) {
    let char_grid = Grid::parse(input, |_pos, c| c);
    let grid = char_grid.map_collect(|_pos, c| match c {
        '#' => Tile::Wall,
        'O' => Tile::Box,
        _ => Tile::Empty,
    });
    let bot_pos = char_grid
        .iter_tiles()
        .find_map(|(pos, c)| match c {
            '@' => Some(pos),
            _ => None,
        })
        .expect("No bot char found");
    (bot_pos, grid)
}

pub fn part_one(input: &str) -> Option<isize> {
    let (grid_text, move_text) = input
        .split_once("\n\n")
        .expect("Cannot split grid and moves");
    let (mut pos, mut grid) = parse_grid(grid_text);
    for c in move_text.chars() {
        // (y,x)
        let off = match c {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => continue,
        };

        // Cast a ray to check if we can move
        let mut tmp = pos;
        let Some(end_pos) = (loop {
            tmp.0 += off.0;
            tmp.1 += off.1;
            match grid.get(tmp) {
                None => break None,
                Some(Tile::Empty) => break Some(tmp),
                Some(Tile::Wall) => break None,
                Some(Tile::Box) => (),
            };
        }) else {
            continue;
        };

        // Move
        pos.0 += off.0;
        pos.1 += off.1;

        // Shift line of boxes if applicable
        if pos != end_pos {
            grid[pos] = Tile::Empty;
            grid[end_pos] = Tile::Box;
        }
    }
    Some(
        grid.iter_tiles()
            .map(|((y, x), tile)| match tile {
                Tile::Box => 100 * y + x,
                _ => 0,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<isize> {
    let (grid_text, move_text) = input
        .split_once("\n\n")
        .expect("Cannot split grid and moves");
    let grid_text = grid_text
        .chars()
        .map(|c| match c {
            '#' => "##".to_string(),
            'O' => "O.".to_string(),
            '.' => "..".to_string(),
            '@' => "@.".to_string(),
            _ => c.to_string(),
        })
        .fold(String::new(), |a, b| a + &b);
    let (mut pos, mut grid) = parse_grid(&grid_text);
    for c in move_text.chars() {
        // (y,x)
        let off = match c {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => continue,
        };

        // Horizontal movement
        if off.0 == 0 {
            // Cast a ray to check if we can move
            let mut tmp = pos;
            let Some(end_pos) = (loop {
                // Vertical component is 0, no need to add
                tmp.1 += off.1;
                match grid.get(tmp) {
                    None => break None,
                    Some(Tile::Empty) if grid.get((tmp.0, tmp.1 - 1)) != Some(&Tile::Box) => {
                        break Some(tmp)
                    }
                    Some(Tile::Empty) => (), // Still part of  a box
                    Some(Tile::Wall) => break None,
                    Some(Tile::Box) => (),
                };
            }) else {
                continue;
            };

            // Move (horizontally)
            pos.1 += off.1;

            // Shift line of boxes if applicable
            if pos != end_pos {
                // We have boxes, move them (weird because boxes are 2 wide)
                for x in pos.1.min(end_pos.1)..pos.1.max(end_pos.1) {
                    let this_tile = &mut grid[(pos.0, x)];
                    if *this_tile == Tile::Box {
                        *this_tile = Tile::Empty;
                    } else {
                        *this_tile = Tile::Box;
                    }
                }
            }
        }
        // Vertical movement
        else {
            // Cast a ray to check if we can move
            let mut already_checked = HashSet::from([pos]);
            let mut queue = vec![pos];
            let mut boxes_pushed = HashSet::new();

            if !loop {
                let Some(mut tmp) = queue.pop() else {
                    // No hazards, move can proceed
                    break true;
                };

                // Horizontal component is 0, no need to add
                tmp.0 += off.0;

                match grid.get(tmp) {
                    None => break false,
                    Some(Tile::Wall) => break false,
                    Some(Tile::Empty) if grid.get((tmp.0, tmp.1 - 1)) != Some(&Tile::Box) => (),
                    Some(Tile::Empty) => {
                        // Still part of a box
                        if !already_checked.contains(&tmp) {
                            queue.push(tmp);
                            already_checked.insert(tmp);
                        }
                        // Do the same for left block of box
                        tmp.1 -= 1;
                        if !already_checked.contains(&tmp) {
                            queue.push(tmp);
                            already_checked.insert(tmp);
                            boxes_pushed.insert(tmp);
                        }
                    }
                    Some(Tile::Box) => {
                        if !already_checked.contains(&tmp) {
                            queue.push(tmp);
                            already_checked.insert(tmp);
                            boxes_pushed.insert(tmp);
                        }
                        // Do the same for right block of box
                        tmp.1 += 1;
                        if !already_checked.contains(&tmp) {
                            queue.push(tmp);
                            already_checked.insert(tmp);
                        }
                    }
                };
            } {
                continue;
            };

            // Move (vertically)
            pos.0 += off.0;

            // Move boxes if applicable
            // First clear boxes' current position, then add then at new position
            for b in boxes_pushed.iter() {
                grid[*b] = Tile::Empty;
            }
            for b in boxes_pushed.iter() {
                grid[(b.0 + off.0, b.1)] = Tile::Box;
            }
        }
    }
    Some(
        grid.iter_tiles()
            .map(|((y, x), tile)| match tile {
                Tile::Box => 100 * y + x,
                _ => 0,
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
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_bis() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9021));
    }
}
