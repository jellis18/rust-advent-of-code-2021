fn get_count_of_increased_depths_by_window(depths: Vec<usize>) -> usize {
    let depth_windows: Vec<usize> = depths.windows(3).map(|w| w.iter().sum::<usize>()).collect();

    depth_windows
        .iter()
        .zip(depth_windows.iter().skip(1))
        .filter(|(x, y)| y > x)
        .count()
}

fn main() {
    let depths: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    println!(
        "Depth window increased {} times",
        get_count_of_increased_depths_by_window(depths)
    )
}

#[test]
fn test_count_increased() {
    let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(5, get_count_of_increased_depths_by_window(depths))
}
