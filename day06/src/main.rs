use std::collections::HashMap;

fn simulate(states: &Vec<u32>, days: u32) -> usize {
    let mut population = HashMap::new();

    // set initial counts of fish at a given state
    for &state in states {
        let count = population.entry(state).or_insert(0);
        *count += 1
    }

    for _ in 0..days {
        let mut next = HashMap::new();
        for age in (1..=8).rev() {
            // decrement all by 1 but don't handle new fish (i.e. 0)
            next.insert(age - 1, *population.get(&age).unwrap_or(&0));
        }

        // all fish at state 0 in the population will spawn new fish with state 8
        next.insert(8, *population.get(&0).unwrap_or(&0));

        // and all fish at state 0 will be reset to state 6, so we add that the
        // existing count of 6s
        let count = next.entry(6).or_insert(0);
        *count += population.get(&0).unwrap_or(&0);

        // replace population with next state
        population = next;
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
