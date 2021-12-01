fn main() {
    let depths: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let count_increased = depths
        .iter()
        .zip(depths.iter().skip(1))
        .filter(|(x, y)| y > x)
        .count();
    println!("Depth increased {0} times", count_increased);
}
