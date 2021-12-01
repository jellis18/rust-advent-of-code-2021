fn get_count_of_increased_depths(depths: Vec<usize>) -> usize {
    depths
        .iter()
        .zip(depths.iter().skip(1))
        .filter(|(x, y)| y > x)
        .count()
}

fn main() {
    let depths: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    println!(
        "Depth increased {0} times",
        get_count_of_increased_depths(depths)
    );
}

#[test]
fn test_count_increased() {
    let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(7, get_count_of_increased_depths(depths))
}
