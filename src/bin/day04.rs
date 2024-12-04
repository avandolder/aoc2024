use itertools::iproduct;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc2024::read_input()?;

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let dimensions = (grid.len(), grid[0].len());

    println!("part one: {}", part_one(&grid, dimensions));
    println!("part two: {}", part_two(&grid, dimensions));

    Ok(())
}

fn search_for(
    grid: &[Vec<char>],
    (row, col): (i64, i64),
    direction: (i64, i64),
    target: &str,
) -> bool {
    (0..target.len() as i64)
        .map(|idx| (row + direction.0 * idx, col + direction.1 * idx))
        .filter(|(row, col)| *row >= 0 && *col >= 0)
        .filter_map(|(row, col)| grid.get(row as usize)?.get(col as usize))
        .collect::<String>()
        == target
}

fn part_one(grid: &[Vec<char>], (rows, cols): (usize, usize)) -> usize {
    iproduct!(0..rows, 0..cols)
        .filter(|(row, col)| grid[*row][*col] == 'X')
        .map(|(row, col)| {
            [
                (0, 1),
                (0, -1),
                (1, 0),
                (-1, 0),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ]
            .into_iter()
            .filter(|direction| search_for(grid, (row as i64, col as i64), *direction, "XMAS"))
            .count()
        })
        .sum()
}

fn part_two(grid: &[Vec<char>], (rows, cols): (usize, usize)) -> usize {
    iproduct!(0..rows, 0..cols)
        .filter(|(row, col)| {
            let (row, col) = (*row as i64, *col as i64);
            (search_for(grid, (row, col), (1, 1), "MAS")
                || search_for(grid, (row, col), (1, 1), "SAM"))
                && (search_for(grid, (row + 2, col), (-1, 1), "MAS")
                    || search_for(grid, (row + 2, col), (-1, 1), "SAM"))
        })
        .count()
}
