advent_of_code::solution!(17);

use enumn::N;
use itertools::Itertools;
use std::rc::Rc;
#[derive(Clone, Debug)]
struct Program {
    regs: RegFile,
    ops: Rc<[u8]>,
}

#[derive(Clone, Copy, Debug)]
struct RegFile {
    a: u64,
    b: u64,
    c: u64,
}

#[repr(u8)]
#[derive(N)]
enum Opcode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

fn parse(input: &str) -> Program {
    let mut l = input.lines();
    let a = l.next().expect("Misssing line A");
    let b = l.next().expect("Misssing line B");
    let c = l.next().expect("Misssing line C");

    let a = a.strip_prefix("Register A: ").expect("Missing prefix A");
    let b = b.strip_prefix("Register B: ").expect("Missing prefix B");
    let c = c.strip_prefix("Register C: ").expect("Missing prefix C");

    let a = a.parse().expect("Parse error A");
    let b = b.parse().expect("Parse error B");
    let c = c.parse().expect("Parse error C");

    l.next();

    let p = Rc::from(
        l.next()
            .expect("Missing program")
            .chars()
            .filter_map(|c| c.to_digit(8).map(|i| i as u8))
            .collect::<Vec<_>>(),
    );

    Program {
        regs: RegFile { a, b, c },
        ops: p,
    }
}

// "Combo" operand value calculation.
fn combo(val: u8, regs: RegFile) -> u64 {
    match val {
        0..=3 => val.into(),
        4 => regs.a,
        5 => regs.b,
        6 => regs.c,
        7 => panic!("Invalid combo operand"),
        _ => unreachable!("Unexpected combo operand value"),
    }
}

fn exec(Program { regs, ops }: &Program) -> Vec<u8> {
    let mut regs = *regs;

    let mut ip = 0;

    let mut out = vec![];

    while let Some(&opcode) = ops.get(ip) {
        let opcode = Opcode::n(opcode).expect("Opcode was unexpectedly out of range!");

        ip += 1;
        let operand = *ops.get(ip).expect("Halted while reading operand!");
        ip += 1;
        match opcode {
            Opcode::Adv => {
                regs.a /= 1 << combo(operand, regs);
            }
            Opcode::Bxl => regs.b ^= u64::from(operand),
            Opcode::Bst => regs.b = combo(operand, regs) % 8,
            Opcode::Jnz => {
                if regs.a != 0 {
                    let new_ip = operand.into();
                    ip = new_ip;
                }
            }
            Opcode::Bxc => {
                regs.b ^= regs.c;
            }
            Opcode::Out => out.push((combo(operand, regs) % 8) as u8),
            Opcode::Bdv => {
                regs.b = regs.a / (1 << combo(operand, regs));
            }
            Opcode::Cdv => {
                regs.c = regs.a / (1 << combo(operand, regs));
            }
        }
    }

    out
}

pub fn part_one(input: &str) -> Option<String> {
    Some(
        exec(&parse(input))
            .into_iter()
            .map(|i| i.to_string())
            .join(","),
    )
}

// https://todd.ginsberg.com/post/advent-of-code/2024/day17/#d17p2
pub fn part_two(input: &str) -> Option<u64> {
    let p = parse(input);
    p.ops
        .iter()
        .rev()
        .enumerate()
        .fold(vec![0u64], |prev_candidates, (i, &instruction)| {
            prev_candidates
                .into_iter()
                .flat_map(|prev_cand| {
                    ((prev_cand << 3)..((prev_cand << 3) + 8)).filter(|cand| {
                        let mut tmp = p.clone();
                        tmp.regs.a = *cand;
                        exec(&tmp).into_iter().rev().nth(i) == Some(instruction)
                    })
                })
                .collect::<Vec<_>>()
        })
        .first()
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
