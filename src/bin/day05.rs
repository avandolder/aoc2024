use std::collections::{HashMap, HashSet};

use aoc::read_input;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input()?;

    let mut rules = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut pages = line.split("|").map(|page| page.parse::<usize>().unwrap());
            (pages.next().unwrap(), pages.next().unwrap())
        })
        .collect::<Vec<_>>();
    rules.sort_unstable_by_key(|(x, _)| *x);

    let updates = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            line.split(",")
                .map(|page| page.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let depends_on: HashMap<usize, HashSet<usize>> = rules
        .iter()
        .chunk_by(|(x, _)| *x)
        .into_iter()
        .map(|(key, chunk)| (key, chunk.into_iter().map(|(_, page)| *page).collect()))
        .collect();

    println!("part one: {}", part_one(&depends_on, &updates));
    println!("part two: {}", part_two(&depends_on, &updates));

    Ok(())
}

fn part_one(depends_on: &HashMap<usize, HashSet<usize>>, updates: &[Vec<usize>]) -> usize {
    updates
        .iter()
        .filter(|update| {
            let mut pages = HashSet::new();
            update.iter().all(|page| {
                if depends_on.contains_key(page) && !depends_on[page].is_disjoint(&pages) {
                    return false;
                }
                pages.insert(*page);
                true
            })
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part_two(depends_on: &HashMap<usize, HashSet<usize>>, updates: &[Vec<usize>]) -> usize {
    updates
        .iter()
        .filter_map(|update| {
            let mut fixed_update = update.to_vec();
            let mut pages = HashSet::new();
            let mut swapped = false;

            'swapped: loop {
                for (i, page) in fixed_update.iter().enumerate() {
                    if depends_on.contains_key(page) && !depends_on[page].is_disjoint(&pages) {
                        let dependent = depends_on[page].intersection(&pages).next().unwrap();
                        let index = fixed_update
                            .iter()
                            .find_position(|page| *page == dependent)
                            .unwrap()
                            .0;
                        fixed_update.swap(index, i);
                        pages.clear();
                        swapped = true;
                        continue 'swapped;
                    }
                    pages.insert(*page);
                }
                break;
            }

            swapped.then_some(fixed_update)
        })
        .map(|update| update[update.len() / 2])
        .sum()
}
