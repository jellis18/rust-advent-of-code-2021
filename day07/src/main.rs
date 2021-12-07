fn get_minimum_fuel_spent(pos: Vec<i32>) -> i32 {
    let n = pos.len();
    let max = *pos.iter().max().unwrap();
    let mut totals = Vec::new();
    for i in 0..max {
        totals.push((0..n).map(|j| (pos[j] - i).abs()).sum());
    }
    totals.into_iter().min().unwrap()
}

fn get_minimum_fuel_spent_part2(pos: Vec<i32>) -> i32 {
    // inefficient and dumb but today was a day :(
    let n = pos.len();
    let max = *pos.iter().max().unwrap();
    let mut totals = Vec::new();
    for i in 0..max {
        totals.push((0..n).map(|j| (0..=(pos[j] - i).abs()).sum::<i32>()).sum());
    }
    totals.into_iter().min().unwrap()
}

fn main() {
    let input: Vec<i32> = include_str!("../input.txt")
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    println!("Minumum fuel spent: {:?}", get_minimum_fuel_spent(input));

    let input: Vec<i32> = include_str!("../input.txt")
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    println!(
        "Minumum fuel spent part2: {:?}",
        get_minimum_fuel_spent_part2(input)
    )
}

#[test]
fn part1() {
    let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    assert_eq!(37, get_minimum_fuel_spent(input))
}

#[test]
fn part2() {
    let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    assert_eq!(168, get_minimum_fuel_spent_part2(input))
}
