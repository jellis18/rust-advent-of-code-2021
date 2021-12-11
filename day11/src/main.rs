use std::collections::HashSet;

const NEIGHBORS: [(isize, isize); 8] = [
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

fn update_map(omap: &mut Vec<Vec<u32>>) -> (Vec<Vec<u32>>, usize) {
    let mut next = omap.clone();
    let mut flashes = Vec::new();
    let mut flash_set = HashSet::new();
    let mut flash_count = 0;
    for i in 0..omap.len() {
        for j in 0..omap[0].len() {
            // energy level increases by 1 for all
            next[i][j] += 1;

            // if energy is now > 9 then add to flashes
            if next[i][j] > 9 {
                flashes.push((i, j));
                flash_set.insert((i, j));
            }
        }
    }

    // loop until all flashes are done
    while !flashes.is_empty() {
        // start at a flash point and update neighbors
        let (i, j) = flashes.pop().unwrap();

        NEIGHBORS
            .iter()
            .map(|(xx, yy)| ((i as isize + xx) as usize, (j as isize + yy) as usize))
            .for_each(|(x, y)| match next.get_mut(x).and_then(|v| v.get_mut(y)) {
                Some(value) => {
                    // increment energy level
                    *value += 1;

                    // check for flash and add if it hasn't flashed yet
                    if (*value > 9) & !flash_set.contains(&(x, y)) {
                        flashes.push((x, y));
                        flash_set.insert((x, y));
                    }
                }

                None => {}
            });
    }

    // clean up and count flashes
    for (i, j) in flash_set {
        next[i][j] = 0;
        flash_count += 1;
    }

    // set map to next
    (next, flash_count)
}

fn solve_part1(omap: &mut Vec<Vec<u32>>, steps: u32) -> usize {
    let mut flash_count = 0;
    for _ in 0..steps {
        let (next, count) = update_map(omap);
        flash_count += count;
        *omap = next;
    }
    flash_count
}

fn solve_part2(omap: &mut Vec<Vec<u32>>) -> usize {
    let total_size = omap.len() * omap[0].len();
    let mut step = 0;
    loop {
        step += 1;

        let (next, count) = update_map(omap);
        if count == total_size {
            return step;
        }
        *omap = next;
    }
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let omap: Vec<Vec<u32>> = input
        .split('\n')
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    omap
}

fn main() {
    let input = include_str!("../input.txt");
    let mut omap = parse_input(input);
    println!("Part 1: {}", solve_part1(&mut omap, 100));

    let input = include_str!("../input.txt");
    let mut omap = parse_input(input);
    println!("Part 2: {}", solve_part2(&mut omap));
}

#[test]
fn part1() {
    let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    let mut omap = parse_input(input);
    assert_eq!(1656, solve_part1(&mut omap, 100));
}

#[test]
fn part2() {
    let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    let mut omap = parse_input(input);
    assert_eq!(195, solve_part2(&mut omap));
}
