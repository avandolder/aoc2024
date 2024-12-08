use std::collections::HashSet;

use aoc::read_input;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input()?;
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (rows, cols) = (map.len() as i64, map[0].len() as i64);

    let antennas: Vec<Vec<(i64, i64)>> = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, c)| ((i, j), c)))
        .filter(|(_, c)| **c != '.')
        .sorted_by_key(|(_, c)| *c)
        .chunk_by(|(_, c)| *c)
        .into_iter()
        .map(|(_, chunk)| {
            chunk
                .into_iter()
                .map(|((i, j), _)| (i as i64, j as i64))
                .collect()
        })
        .collect();

    println!("part one: {}", part_one((rows, cols), &antennas));
    println!("part two: {}", part_two((rows, cols), &antennas));

    Ok(())
}

fn part_one((rows, cols): (i64, i64), antennas: &[Vec<(i64, i64)>]) -> usize {
    let mut antinodes = HashSet::new();

    for locs in antennas {
        for i in 0..locs.len() {
            for j in (i + 1)..locs.len() {
                let (row1, col1) = locs[i];
                let (row2, col2) = locs[j];

                let (dr, dc) = (row1 - row2, col1 - col2);
                antinodes.insert((row1 + dr, col1 + dc));
                antinodes.insert((row2 - dr, col2 - dc));
            }
        }
    }

    antinodes
        .into_iter()
        .filter(|(row, col)| 0 <= *row && *row < rows && 0 <= *col && *col < cols)
        .count()
}

fn part_two((rows, cols): (i64, i64), antennas: &[Vec<(i64, i64)>]) -> usize {
    let mut antinodes = HashSet::new();

    for locs in antennas {
        for i in 0..locs.len() {
            for j in (i + 1)..locs.len() {
                let (row1, col1) = locs[i];
                let (row2, col2) = locs[j];
                let (dr, dc) = (row1 - row2, col1 - col2);

                let mut r = row1;
                let mut c = col1;
                while 0 <= r && r < rows && 0 <= c && c < cols {
                    antinodes.insert((r, c));
                    r += dr;
                    c += dc;
                }

                let mut r = row2;
                let mut c = col2;
                while 0 <= r && r < rows && 0 <= c && c < cols {
                    antinodes.insert((r, c));
                    r -= dr;
                    c -= dc;
                }
            }
        }
    }

    antinodes.len()
}
