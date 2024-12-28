use std::cmp::Ordering;

use aoc::read_input;
use regex::Regex;

const W: i64 = 101;
const H: i64 = 103;

#[derive(Clone, Debug)]
struct Robot {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

impl From<((i64, i64), (i64, i64))> for Robot {
    fn from(((x, y), (dx, dy)): ((i64, i64), (i64, i64))) -> Self {
        Self { x, y, dx, dy }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let robot_regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")?;
    let robots: Vec<Robot> = read_input()?
        .lines()
        .map(|line| {
            let mut ns = robot_regex
                .captures(line)
                .unwrap()
                .extract::<4>()
                .1
                .into_iter()
                .map(|n| n.parse().unwrap());
            (
                (ns.next().unwrap(), ns.next().unwrap()),
                (ns.next().unwrap(), ns.next().unwrap()),
            )
                .into()
        })
        .collect();

    println!("part one: {}", part_one(&robots));
    // println!("part two: {}", part_two(&grid, dim));

    Ok(())
}

fn part_one(robots: &[Robot]) -> i64 {
    let mut quads = [0, 0, 0, 0];

    for Robot { x, y, dx, dy } in robots {
        let (x, y) = ((x + dx * 100).rem_euclid(W), (y + dy * 100).rem_euclid(H));

        use Ordering::{Greater, Less};
        match (x.cmp(&(W / 2)), y.cmp(&(H / 2))) {
            (Less, Less) => quads[0] += 1,
            (Less, Greater) => quads[1] += 1,
            (Greater, Less) => quads[2] += 1,
            (Greater, Greater) => quads[3] += 1,
            _ => (),
        }
    }

    quads.iter().product()
}
