use aoc::read_input;
use num::{bigint::BigUint, FromPrimitive};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input()?;

    let eqns = input
        .lines()
        .map(|line| {
            let mut parts = line.split(":");
            (
                parts.next().unwrap().parse::<u64>().unwrap(),
                parts
                    .next()
                    .unwrap()
                    .split(" ")
                    .filter(|str| !str.is_empty())
                    .map(|value| value.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    println!("part one: {}", part_one(&eqns));
    println!("part two: {}", part_two(&eqns));

    Ok(())
}

fn can_get(result: u64, n: u64, values: &[u64]) -> bool {
    let x = match values {
        [] => return result == n,
        [x, ..] => *x,
    };

    can_get(result, n + x, &values[1..]) || can_get(result, n * x, &values[1..])
}

fn part_one(eqns: &[(u64, Vec<u64>)]) -> u64 {
    eqns.iter()
        .filter_map(|(result, values)| can_get(*result, values[0], &values[1..]).then_some(result))
        .sum()
}

fn can_get_big(result: &BigUint, n: BigUint, values: &[u64]) -> bool {
    let x: BigUint = match values {
        [] => return *result == n,
        [x, ..] => (*x).into(),
    };

    can_get_big(result, &n + &x, &values[1..]) || can_get_big(result, &n * &x, &values[1..]) || {
        let x = x.to_string();
        let n = (n.to_string() + &x).parse().unwrap();
        can_get_big(result, n, &values[1..])
    }
}

fn part_two(eqns: &[(u64, Vec<u64>)]) -> u64 {
    eqns.iter()
        .filter_map(|(result, values)| {
            can_get_big(
                &BigUint::from_u64(*result).unwrap(),
                BigUint::from_u64(values[0]).unwrap(),
                &values[1..],
            )
            .then_some(result)
        })
        .sum()
}
