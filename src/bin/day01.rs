use std::collections::HashMap;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut xs, mut ys): (Vec<_>, Vec<_>) = std::io::stdin()
        .lines()
        .flat_map(|line| {
            let line = line?;
            let mut ids = line.split_whitespace();
            Ok::<_, Box<dyn std::error::Error>>((
                ids.next().unwrap().parse::<i32>()?,
                ids.next().unwrap().parse::<i32>()?,
            ))
        })
        .unzip();
    xs.sort_unstable();
    ys.sort_unstable();

    {
        let total_dist = std::iter::zip(&xs, &ys)
            .map(|(x, y)| (*x - *y).abs())
            .sum::<i32>();
        println!("part one: {}", total_dist);
    }

    {
        let counts = ys.into_iter()
            .chunk_by(|elt| *elt)
            .into_iter()
            .map(|(key, group)| (key, group.count() as i32))
            .collect::<HashMap<_, _>>();
        let score = xs.into_iter()
            .map(|x| x * counts.get(&x).unwrap_or(&0))
            .sum::<i32>();
        println!("part two: {}", score);
    }

    Ok(())
}
