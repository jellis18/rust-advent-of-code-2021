const POSITIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let x: Vec<Vec<u32>> = input
        .split('\n')
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    x
}

fn solve_part1(x: Vec<Vec<u32>>) -> u32 {
    let nrow = x.len();
    let ncol = x[0].len();
    let mut risk_level = 0;
    for i in 0..nrow {
        for j in 0..ncol {
            let vertical_points = if i == 0 {
                x[i][j] < x[i + 1][j]
            } else if i == (nrow - 1) {
                x[i][j] < x[i - 1][j]
            } else {
                (x[i][j] < x[i + 1][j]) & (x[i][j] < x[i - 1][j])
            };

            let horizontal_points = if j == 0 {
                x[i][j] < x[i][j + 1]
            } else if j == (ncol - 1) {
                x[i][j] < x[i][j - 1]
            } else {
                (x[i][j] < x[i][j + 1]) & (x[i][j] < x[i][j - 1])
            };

            if (vertical_points == true) & (horizontal_points == true) {
                risk_level += x[i][j] + 1;
            }
        }
    }
    risk_level
}

fn solve_part2(mapping: &mut Vec<Vec<u32>>) -> usize {
    let mut basins = Vec::new();
    for i in 0..mapping.len() {
        for j in 0..mapping[0].len() {
            if mapping[i][j] < 9 {
                basins.push(get_basins(mapping, i, j));
            }
        }
    }
    basins.sort();
    basins.iter().rev().take(3).product()
}

fn get_basins(mapping: &mut Vec<Vec<u32>>, i: usize, j: usize) -> usize {
    // set current position as wall so we can't go back and double count
    mapping[i][j] = 9;
    POSITIONS
        .iter()
        .map(|(xx, yy)| ((i as isize + xx) as usize, (j as isize + yy) as usize))
        .fold(1, |acc, (x, y)| {
            match mapping.get(x).and_then(|v| v.get(y)).map(|&c| c < 9) {
                Some(true) => acc + get_basins(mapping, x, y),
                _ => acc,
            }
        })
}

fn main() {
    let input = include_str!("../input.txt");
    let x = parse_input(input);
    println!("Part 1 {:?}", solve_part1(x));

    let input = include_str!("../input.txt");
    let mut x = parse_input(input);
    println!("Part 2 {}", solve_part2(&mut x));
}

#[test]
fn part1() {
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
    let x = parse_input(input);
    assert_eq!(15, solve_part1(x));
}

#[test]
fn part2() {
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
    let mut x = parse_input(input);
    assert_eq!(1134, solve_part2(&mut x));
}
