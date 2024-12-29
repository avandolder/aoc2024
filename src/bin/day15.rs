use aoc::read_input;
use itertools::iproduct;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input()?;

    let grid: Vec<Vec<char>> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    let dim = (grid.len(), grid[0].len());

    let instructions: Vec<(i64, i64)> = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '>' => (1, 0),
            '<' => (-1, 0),
            'v' => (0, 1),
            '^' => (0, -1),
            _ => panic!(),
        })
        .collect();

    println!("part one: {}", part_one(&grid, dim, &instructions));
    println!("part two: {}", part_two(&grid, dim, &instructions));

    Ok(())
}

fn part_one(grid: &[Vec<char>], (w, h): (usize, usize), instructions: &[(i64, i64)]) -> usize {
    fn try_move(
        grid: &mut [Vec<char>],
        (w, h): (usize, usize),
        (x, y): (usize, usize),
        (dx, dy): (i64, i64),
    ) -> bool {
        if x as i64 + dx < 0
            || x as i64 + dx >= w as i64
            || y as i64 + dy < 0
            || y as i64 + dy >= h as i64
        {
            return false;
        }
        let (xx, yy) = ((x as i64 + dx) as usize, (y as i64 + dy) as usize);

        match grid[yy][xx].clone() {
            'O' if try_move(grid, (w, h), (xx, yy), (dx, dy)) => {
                grid[yy][xx] = grid[y][x];
                true
            }
            '.' => {
                grid[yy][xx] = grid[y][x];
                true
            }
            _ => false,
        }
    }

    let mut grid = grid.to_vec();
    let (mut x, mut y) = iproduct!(0..w, 0..h)
        .find(|(x, y)| grid[*y][*x] == '@')
        .unwrap();

    for (dx, dy) in instructions.iter().copied() {
        if try_move(&mut grid, (w, h), (x, y), (dx, dy)) {
            grid[y][x] = '.';
            x = (x as i64 + dx) as usize;
            y = (y as i64 + dy) as usize;
        }
    }

    iproduct!(0..w, 0..h)
        .filter(|(x, y)| grid[*y][*x] == 'O')
        .map(|(x, y)| y * 100 + x)
        .sum()
}

fn part_two(_grid: &[Vec<char>], (_w, _h): (usize, usize), _instructions: &[(i64, i64)]) -> usize {
    0
}
