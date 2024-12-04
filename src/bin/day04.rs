use itertools::iproduct;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoc2024::read_input()?;

    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));

    Ok(())
}

fn search_for(
    grid: &[Vec<char>],
    (rows, cols): (i64, i64),
    (row, col): (i64, i64),
    direction: (i64, i64),
    target: &'static str,
) -> bool {
    (0..target.len() as i64)
        .map(|idx| (row + direction.0 * idx, col + direction.1 * idx))
        .filter(|(row, col)| *row >= 0 && *row < rows && *col >= 0 && *col < cols)
        .map(|(row, col)| grid[row as usize][col as usize])
        .collect::<String>()
        == target
}

fn part_one(input: &str) -> i64 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let dimensions = (grid.len() as i64, grid[0].len() as i64);

    iproduct!(0..dimensions.0, 0..dimensions.1)
        .filter(|(row, col)| grid[*row as usize][*col as usize] == 'X')
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
            .filter(|direction| search_for(&grid, dimensions, (row, col), *direction, "XMAS"))
            .count() as i64
        })
        .sum()
}

fn part_two(input: &str) -> i64 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let dimensions = (grid.len() as i64, grid[0].len() as i64);

    iproduct!(0..dimensions.0, 0..dimensions.1)
        .filter(|(row, col)| {
            let (row, col) = (*row, *col);
            (search_for(&grid, dimensions, (row, col), (1, 1), "MAS")
                || search_for(&grid, dimensions, (row, col), (1, 1), "SAM"))
                && (search_for(&grid, dimensions, (row + 2, col), (-1, 1), "MAS")
                    || search_for(&grid, dimensions, (row + 2, col), (-1, 1), "SAM"))
        })
        .count() as i64
}
