use std::collections::HashMap;

use aoc::read_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input()?;
    let stones: Vec<i64> = input
        .split_ascii_whitespace()
        .map(|stone| stone.parse().unwrap())
        .collect();

    println!("part one: {}", part_one(&stones));
    println!("part two: {}", part_two(&stones));

    Ok(())
}

fn count_stones<const MAX_STEPS: i64>(
    cache: &mut HashMap<(i64, i64), i64>,
    stone: i64,
    step: i64,
) -> i64 {
    if step == MAX_STEPS {
        return 1;
    }
    if let Some(value) = cache.get(&(stone, step)) {
        return *value;
    }

    let value = if stone == 0 {
        count_stones::<MAX_STEPS>(cache, 1, step + 1)
    } else {
        let digits = stone.checked_ilog10().unwrap() + 1;
        if digits % 2 == 0 {
            let pow = 10i64.pow(digits / 2);
            count_stones::<MAX_STEPS>(cache, stone / pow, step + 1)
                + count_stones::<MAX_STEPS>(cache, stone % pow, step + 1)
        } else {
            count_stones::<MAX_STEPS>(cache, stone * 2024, step + 1)
        }
    };
    cache.insert((stone, step), value);
    value
}

fn part_one(stones: &[i64]) -> i64 {
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|stone| count_stones::<25>(&mut cache, *stone, 0))
        .sum()
}

fn part_two(stones: &[i64]) -> i64 {
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|stone| count_stones::<75>(&mut cache, *stone, 0))
        .sum()
}
