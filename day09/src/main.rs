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

fn solve_part1(mapping: Vec<Vec<u32>>) -> u32 {
    let mut height = 0;
    for i in 0..mapping.len() {
        for j in 0..mapping[0].len() {
            let is_low_point = POSITIONS
                .iter()
                .map(|(xx, yy)| ((i as isize + xx) as usize, (j as isize + yy) as usize))
                .all(|(x, y)| {
                    mapping
                        .get(x)
                        .and_then(|v| v.get(y))
                        .map(|&c| c > mapping[i][j])
                        .unwrap_or(true)
                });
            if is_low_point == true {
                height += mapping[i][j] + 1;
            }
        }
    }
    height
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
