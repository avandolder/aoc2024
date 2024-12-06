use std::collections::HashSet;

use aoc::read_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input()?;

    let obstructions = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(j, _)| (i as i64, j as i64))
        })
        .collect::<HashSet<(i64, i64)>>();
    let start = input
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            line.chars()
                .enumerate()
                .find(|(_, c)| *c == '^')
                .map(move |(j, _)| (i as i64, j as i64))
        })
        .next()
        .unwrap();
    let (rows, cols) = (
        input.lines().count() as i64,
        input.lines().next().unwrap().len() as i64,
    );

    println!("part one: {}", part_one((rows, cols), &obstructions, start));
    println!("part two: {}", part_two((rows, cols), &obstructions, start));

    Ok(())
}

fn turn_right(direction: (i64, i64)) -> (i64, i64) {
    match direction {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => unreachable!(),
    }
}

fn part_one(
    (rows, cols): (i64, i64),
    obstructions: &HashSet<(i64, i64)>,
    (mut i, mut j): (i64, i64),
) -> usize {
    let mut direction = (-1, 0);
    let mut positions = HashSet::new();

    while i >= 0 && i < rows && j >= 0 && j < cols {
        positions.insert((i, j));

        if obstructions.contains(&(i + direction.0, j + direction.1)) {
            direction = turn_right(direction);
        } else {
            i += direction.0;
            j += direction.1;
        }
    }

    positions.len()
}

fn part_two(
    (rows, cols): (i64, i64),
    obstructions: &HashSet<(i64, i64)>,
    (start_i, start_j): (i64, i64),
) -> usize {
    let mut direction = (-1, 0);
    let mut positions = HashSet::new();
    let (mut i, mut j) = (start_i, start_j);

    while i >= 0 && i < rows && j >= 0 && j < cols {
        positions.insert(((i, j), direction));

        if obstructions.contains(&(i + direction.0, j + direction.1)) {
            direction = turn_right(direction);
        } else {
            i += direction.0;
            j += direction.1;
        }
    }

    let mut new_obstructions = HashSet::new();
    for ((mut i, mut j), mut direction) in &positions {
        let (new_i, new_j) = (i + direction.0, j + direction.1);
        if obstructions.contains(&(new_i, new_j))
            || new_i < 0
            || new_i >= rows
            || new_j < 0
            || new_j >= cols
            || (new_i == start_i && new_j == start_j)
        {
            continue;
        }

        let mut obstructions = obstructions.clone();
        obstructions.insert((new_i, new_j));

        i = start_i;
        j = start_j;
        direction = (-1, 0);
        let mut positions = HashSet::new();
        while i >= 0 && i < rows && j >= 0 && j < cols {
            if positions.contains(&((i, j), direction)) {
                new_obstructions.insert((new_i, new_j));
                break;
            }

            positions.insert(((i, j), direction));
            if obstructions.contains(&(i + direction.0, j + direction.1)) {
                direction = turn_right(direction);
            } else {
                i += direction.0;
                j += direction.1;
            }
        }
    }

    new_obstructions.len()
}
