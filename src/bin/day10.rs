use std::collections::HashSet;

use aoc::read_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input()?;
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let dim = (grid.len(), grid[0].len());

    println!("part one: {}", part_one(&grid, dim));
    println!("part two: {}", part_two(&grid, dim));

    Ok(())
}

fn part_one(grid: &[Vec<u32>], dim: (usize, usize)) -> usize {
    fn walk(
        grid: &[Vec<u32>],
        ends: &mut HashSet<(usize, usize)>,
        (w, h): (usize, usize),
        (i, j): (usize, usize),
    ) {
        if grid[i][j] == 9 {
            ends.insert((i, j));
            return;
        }
        [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .into_iter()
            .map(|(di, dj)| (i as i64 + di, j as i64 + dj))
            .filter(|(ii, jj)| {
                *ii >= 0
                    && *ii < w as i64
                    && *jj >= 0
                    && *jj < h as i64
                    && grid[i][j] + 1 == grid[*ii as usize][*jj as usize]
            })
            .for_each(|(i, j)| walk(grid, ends, (w, h), (i as usize, j as usize)))
    }

    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, x)| **x == 0)
                .map(move |(j, _)| (i, j))
        })
        .map(|loc| {
            let mut ends = HashSet::new();
            walk(grid, &mut ends, dim, loc);
            ends.len()
        })
        .sum()
}

fn part_two(grid: &[Vec<u32>], dim: (usize, usize)) -> i64 {
    fn walk(grid: &[Vec<u32>], (w, h): (usize, usize), (i, j): (usize, usize)) -> i64 {
        if grid[i][j] == 9 {
            return 1;
        }
        [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .into_iter()
            .map(|(di, dj)| (i as i64 + di, j as i64 + dj))
            .filter(|(ii, jj)| {
                *ii >= 0
                    && *ii < w as i64
                    && *jj >= 0
                    && *jj < h as i64
                    && grid[i][j] + 1 == grid[*ii as usize][*jj as usize]
            })
            .map(|(i, j)| walk(grid, (w, h), (i as usize, j as usize)))
            .sum()
    }

    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, x)| **x == 0)
                .map(move |(j, _)| (i, j))
        })
        .map(|loc| walk(grid, dim, loc))
        .sum()
}

