use std::collections::HashMap;

fn simulate(states: &Vec<u32>, days: u32) -> usize {
    let mut population = HashMap::new();

    // set initial counts of fish at a given state
    for state in states {
        let count = population.entry(*state).or_insert(0);
        *count += 1
    }

    for _ in 0..days {
        let mut next_generation = HashMap::new();
        for age in (1..=8).rev() {
            // decrement
            next_generation.insert(age - 1, *population.get(&age).unwrap_or_else(|| &0));
        }
        // All 0s will add an 8
        next_generation.insert(8, *population.get(&0).unwrap_or_else(|| &0));

        // and add
        let count = next_generation.entry(6).or_insert(0);
        *count += population.get(&0).unwrap_or_else(|| &0);

        // replace population with next generation
        population = next_generation;
    }
    population.values().sum()
}
fn main() {
    let states: Vec<u32> = include_str!("../input.txt")
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    println!("Number of fish after 80 days: {}", simulate(&states, 80));
    println!("Number of fish after 256 days: {}", simulate(&states, 256));
}

#[test]
fn part1() {
    let states: Vec<u32> = "3,4,3,1,2"
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    assert_eq!(5934, simulate(&states, 80))
}

#[test]
fn part2() {
    let states: Vec<u32> = "3,4,3,1,2"
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    assert_eq!(26984457539, simulate(&states, 256))
}
