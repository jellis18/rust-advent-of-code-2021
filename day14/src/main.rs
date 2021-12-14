use std::collections::HashMap;

fn parse_input(
    input: &str,
) -> (
    HashMap<(char, char), char>,
    HashMap<char, isize>,
    HashMap<(char, char), isize>,
) {
    let mut lines = input.lines();

    // keep track of three hashmaps
    // 1. A lookup for the insertion rules
    // 2. A counter to keep track of the character counts
    // 3. A counter to keep track of the pair counts
    let mut rules = HashMap::new();
    let mut character_counter = HashMap::new();
    let mut pair_counter = HashMap::new();

    let template = lines.next().unwrap();
    for (c1, c2) in template.chars().zip(template.chars().skip(1)) {
        *pair_counter.entry((c1, c2)).or_insert(0) += 1;
    }

    for c in template.chars() {
        *character_counter.entry(c).or_insert(0) += 1;
    }

    // empty space
    lines.next();

    // this feels idiotic but it works and I'm in a rush
    loop {
        match lines.next() {
            Some(v) => {
                let (pair, insert) = v.split_once(" -> ").unwrap();
                let mut pair_chars = pair.chars();
                rules.insert(
                    (pair_chars.next().unwrap(), pair_chars.next().unwrap()),
                    insert.chars().next().unwrap(),
                );
            }
            None => return (rules, character_counter, pair_counter),
        }
    }
}

fn solve(
    rules: &HashMap<(char, char), char>,
    mut character_counter: HashMap<char, isize>,
    mut pair_counter: HashMap<(char, char), isize>,
    steps: usize,
) -> usize {
    for _ in 0..steps {
        let mut next_pair_counter = HashMap::new();
        for ((c1, c2), ct) in pair_counter {
            // look insertion value
            let insert = rules.get(&(c1, c2)).unwrap();

            // update the pair counters
            *next_pair_counter.entry((c1, *insert)).or_insert(0) += ct;
            *next_pair_counter.entry((*insert, c2)).or_insert(0) += ct;

            // update character counter
            *character_counter.entry(*insert).or_insert(0) += ct;
        }
        pair_counter = next_pair_counter;
    }
    let max = character_counter.values().max().unwrap();
    let min = character_counter.values().min().unwrap();
    (max - min) as usize
}

fn main() {
    let input = include_str!("../input.txt");
    let (rules, character_counter, pair_counter) = parse_input(input);
    println!(
        "Part 1: {}",
        solve(&rules, character_counter, pair_counter, 10)
    );

    let (rules, character_counter, pair_counter) = parse_input(input);
    println!(
        "Part 2: {}",
        solve(&rules, character_counter, pair_counter, 40)
    );
}

#[test]
fn part1() {
    let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    let (rules, character_counter, pair_counter) = parse_input(input);
    assert_eq!(1588, solve(&rules, character_counter, pair_counter, 10))
}

#[test]
fn part2() {
    let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    let (rules, character_counter, pair_counter) = parse_input(input);
    assert_eq!(
        2188189693529,
        solve(&rules, character_counter, pair_counter, 40)
    )
}
