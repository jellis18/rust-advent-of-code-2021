use std::collections::HashMap;

fn get_most_and_least_common_bit(x: Vec<u32>) -> (u32, u32) {
    let counter = x.iter().fold(HashMap::<u32, usize>::new(), |mut m, x| {
        *m.entry(*x).or_default() += 1;
        m
    });

    let most_common = counter
        .iter()
        .max_by_key(|(_, v)| *v)
        .map(|(k, _)| k)
        .unwrap();

    let least_common = counter
        .iter()
        .min_by_key(|(_, v)| *v)
        .map(|(k, _)| k)
        .unwrap();

    (*most_common, *least_common)
}

fn get_product_of_rates(input: Vec<&str>) -> u32 {
    let x: Vec<Vec<u32>> = input
        .iter()
        .map(|x| {
            x.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect();

    // get length
    let n = x[0].len();
    let mut gamma_rate_str = "".to_owned();
    let mut epsilon_rate_str = "".to_owned();
    for ii in 0..n {
        let mut columns = Vec::new();
        for v in x.iter() {
            columns.push(v[ii]);
        }
        let (m, l) = get_most_and_least_common_bit(columns);
        gamma_rate_str.push_str(m.to_string().as_str());
        epsilon_rate_str.push_str(l.to_string().as_str());
    }
    let gamma_rate = u32::from_str_radix(gamma_rate_str.as_str(), 2).unwrap();
    let epsilon_rate = u32::from_str_radix(epsilon_rate_str.as_str(), 2).unwrap();
    gamma_rate * epsilon_rate
}

fn main() {
    let input = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    println!("Power consumption is {}", get_product_of_rates(input))
}

#[test]
fn day03a_sample() {
    let input = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];
    assert_eq!(198, get_product_of_rates(input));
}
