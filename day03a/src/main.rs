fn get_power_consumption(input: Vec<&str>) -> u32 {
    // get length
    let n = input[0].len();
    let mut gamma_rate_str = "".to_owned();
    let mut epsilon_rate_str = "".to_owned();
    for location in 0..n {
        let (zero_count, one_count) = find_counts_in_bit_location(&input, location);
        if zero_count > one_count {
            gamma_rate_str.push_str("0");
            epsilon_rate_str.push_str("1");
        } else {
            gamma_rate_str.push_str("1");
            epsilon_rate_str.push_str("0");
        }
    }
    let gamma_rate = u32::from_str_radix(gamma_rate_str.as_str(), 2).unwrap();
    let epsilon_rate = u32::from_str_radix(epsilon_rate_str.as_str(), 2).unwrap();
    gamma_rate * epsilon_rate
}

fn find_counts_in_bit_location(x: &Vec<&str>, location: usize) -> (usize, usize) {
    let steps: String = x.iter().filter_map(|s| s.chars().nth(location)).collect();
    (steps.matches("0").count(), steps.matches("1").count())
}

fn main() {
    let input = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    println!("Power consumption is {}", get_power_consumption(input))
}

#[test]
fn day03a_sample() {
    let input = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];
    assert_eq!(198, get_power_consumption(input));
}
