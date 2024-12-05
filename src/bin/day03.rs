use aoc::read_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input()?;

    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));

    Ok(())
}

fn part_one(mut src: &str) -> i64 {
    let mut total = 0i64;

    while src.len() >= 4 {
        match &src[..4] {
            "mul(" => {
                src = &src[4..];

                let digits = src.chars().take_while(|c| c.is_ascii_digit()).count();
                if digits == 0 {
                    continue;
                }
                let lhs = src[..digits].parse::<i64>().unwrap();

                src = &src[digits..];
                match src.chars().next() {
                    Some(',') => src = &src[1..],
                    _ => continue,
                }

                let digits = src.chars().take_while(|c| c.is_ascii_digit()).count();
                if digits == 0 {
                    continue;
                }
                let rhs = src[..digits].parse::<i64>().unwrap();

                match src.chars().nth(digits) {
                    Some(')') => total += lhs * rhs,
                    _ => continue,
                }
                src = &src[digits + 1..]
            }
            _ => src = &src[1..],
        }
    }

    total
}

fn part_two(mut src: &str) -> i64 {
    let mut enabled = true;
    let mut total = 0i64;

    while src.len() >= 4 {
        if src.starts_with("don't()") {
            enabled = false;
            src = &src[7..];
            continue;
        }

        match &src[..4] {
            "mul(" => {
                src = &src[4..];
                if !enabled {
                    continue;
                }

                let digits = src.chars().take_while(|c| c.is_ascii_digit()).count();
                if digits == 0 {
                    continue;
                }
                let lhs = src[..digits].parse::<i64>().unwrap();

                src = &src[digits..];
                match src.chars().next() {
                    Some(',') => src = &src[1..],
                    _ => continue,
                }

                let digits = src.chars().take_while(|c| c.is_ascii_digit()).count();
                if digits == 0 {
                    continue;
                }
                let rhs = src[..digits].parse::<i64>().unwrap();

                match src.chars().nth(digits) {
                    Some(')') => total += lhs * rhs,
                    _ => continue,
                }
                src = &src[digits + 1..]
            }
            "do()" => {
                enabled = true;
                src = &src[4..];
            }
            _ => src = &src[1..],
        }
    }

    total
}
