use std::collections::HashSet;

use aoc::read_input;
use itertools::iproduct;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input()?;
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let dim = (grid.len(), grid[0].len());

    println!("part one: {}", part_one(&grid, dim));
    println!("part two: {}", part_two(&grid, dim));

    Ok(())
}

fn part_one(grid: &[Vec<char>], (w, h): (usize, usize)) -> usize {
    fn dfs(
        seen: &mut HashSet<(usize, usize)>,
        grid: &[Vec<char>],
        (w, h): (usize, usize),
        kind: char,
        (i, j): (usize, usize),
    ) -> (usize, usize) {
        if seen.contains(&(i, j)) {
            return (0, 0);
        }
        seen.insert((i, j));

        let (peri, area) = [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .into_iter()
            .map(|(di, dj)| (di + i as i64, dj + j as i64))
            .map(|(i, j)| {
                if i < 0
                    || i >= w as i64
                    || j < 0
                    || j >= h as i64
                    || grid[i as usize][j as usize] != kind
                {
                    return (1, 0);
                }
                dfs(seen, grid, (w, h), kind, (i as usize, j as usize))
            })
            .fold((0, 0), |(peri, area), acc| (acc.0 + peri, acc.1 + area));
        (peri, area + 1)
    }

    let mut seen = HashSet::with_capacity(w * h);
    iproduct!(0..w, 0..h)
        .map(|(i, j)| {
            let (peri, area) = dfs(&mut seen, grid, (w, h), grid[i][j], (i, j));
            peri * area
        })
        .sum()
}

fn part_two(_grid: &[Vec<char>], _dim: (usize, usize)) -> usize {
    0
}
