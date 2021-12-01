fn main() {
    let depth_windows: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .windows(3)
        .map(|w| w.iter().sum::<usize>())
        .collect();

    let depth_window_increased = depth_windows
        .iter()
        .zip(depth_windows.iter().skip(1))
        .filter(|(x, y)| y > x)
        .count();

    println!("Depth window increased {} times", depth_window_increased)
}
