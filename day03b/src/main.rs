fn find_counts_in_bit_location(x: &Vec<&str>, location: usize) -> (usize, usize) {
    let steps: String = x.iter().filter_map(|s| s.chars().nth(location)).collect();
    (steps.matches("0").count(), steps.matches("1").count())
}

fn get_oxygen_generator_rating(x: &Vec<&str>) -> usize {
    let n = x[0].len();
    let mut samples = x.clone();
    for location in 0..n {
        if samples.len() == 1 {
            break;
        }

        let (zero_count, one_count) = find_counts_in_bit_location(&samples, location);
        // find most frequent bit
        if zero_count > one_count {
            samples.retain(|x| x.chars().nth(location).unwrap() == '0');
        } else {
            samples.retain(|x| x.chars().nth(location).unwrap() == '1');
        }
    }
    usize::from_str_radix(samples[0], 2).unwrap()
}
fn get_co2_scrubber_rating_rating(x: &Vec<&str>) -> usize {
    let n = x[0].len();
    let mut samples = x.clone();
    for location in 0..n {
        if samples.len() == 1 {
            break;
        }

        let (zero_count, one_count) = find_counts_in_bit_location(&samples, location);
        // find least frequent bit
        if zero_count > one_count {
            samples.retain(|x| x.chars().nth(location).unwrap() == '1');
        } else {
            samples.retain(|x| x.chars().nth(location).unwrap() == '0');
        }
    }
    usize::from_str_radix(samples[0], 2).unwrap()
}

fn get_life_support_rating(x: &Vec<&str>) -> usize {
    get_co2_scrubber_rating_rating(x) * get_oxygen_generator_rating(x)
}

fn main() {
    let input = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    println!(
        "Life support rating is  is {}",
        get_life_support_rating(&input)
    )
}

#[test]
fn day03b_sample() {
    let input = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];
    assert_eq!(230, get_life_support_rating(&input));
}
