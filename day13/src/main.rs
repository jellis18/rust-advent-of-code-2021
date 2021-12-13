use std::collections::HashSet;

#[derive(PartialEq, Debug)]
enum Fold {
    X(isize),
    Y(isize),
}

fn visualize_map(map: &HashSet<(isize, isize)>) {
    // get size of map
    let num_rows = *map.iter().map(|(_, y)| y).max().unwrap();
    let num_cols = *map.iter().map(|(x, _)| x).max().unwrap();
    let mut vis = "".to_owned();
    for y in 0..=num_rows {
        for x in 0..=num_cols {
            match map.get(&(x, y)) {
                Some(_) => vis.push('#'),
                None => vis.push('.'),
            }
        }
        vis.push('\n')
    }
    println!("{}", vis);
}

fn parse_input(input: &str) -> (HashSet<(isize, isize)>, Vec<Fold>) {
    let mut lines = input.lines();

    let mut map = HashSet::new();
    let mut folds = Vec::new();
    loop {
        match lines.next() {
            Some(v) if !v.starts_with("fold") && v != "" => {
                map.insert(
                    v.split_once(',')
                        .map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
                        .unwrap(),
                );
            }
            Some(v) if v.starts_with("fold") => {
                let (axis, value) = v
                    .split_once("fold along ")
                    .unwrap()
                    .1
                    .split_once('=')
                    .unwrap();
                if axis == "x" {
                    folds.push(Fold::X(value.parse::<isize>().unwrap()));
                } else {
                    folds.push(Fold::Y(value.parse::<isize>().unwrap()));
                }
            }
            Some(_) => {}
            None => return (map, folds),
        }
    }
}

fn fold_along_y(map: &mut HashSet<(isize, isize)>, y_fold: isize) {
    // find number of rows
    let num_rows = *map.iter().map(|(_, y)| y).max().unwrap();
    for (ct, i) in ((y_fold + 1)..=num_rows).enumerate() {
        // get transformed y coordinate after fold
        let y = i - ((ct as isize) * 2 + 2);

        // drop off pre-folded points from map and add new transformed points
        let pre_folded: Vec<(isize, isize)> =
            map.iter().cloned().filter(|(_, yy)| *yy == i).collect();
        for (xx, yy) in pre_folded {
            map.remove(&(xx, yy));
            map.insert((xx, y));
        }
    }
}

fn fold_along_x(map: &mut HashSet<(isize, isize)>, x_fold: isize) {
    // find number of rows
    let num_cols = *map.iter().map(|(x, _)| x).max().unwrap();
    for (ct, i) in ((x_fold + 1)..=num_cols).enumerate() {
        // get transformed y coordinate after fold
        let x = i - ((ct as isize) * 2 + 2);

        // drop off pre-folded points from map and add new transformed points
        let pre_folded: Vec<(isize, isize)> =
            map.iter().cloned().filter(|(xx, _)| *xx == i).collect();
        for (xx, yy) in pre_folded {
            map.remove(&(xx, yy));
            map.insert((x, yy));
        }
    }
}

fn solve_part1(map: &mut HashSet<(isize, isize)>, folds: Vec<Fold>) -> usize {
    match folds[0] {
        Fold::X(v) => fold_along_x(map, v),
        Fold::Y(v) => fold_along_y(map, v),
    }
    map.len()
}

fn solve_part2(map: &mut HashSet<(isize, isize)>, folds: Vec<Fold>) {
    folds.into_iter().for_each(|fold| match fold {
        Fold::X(v) => fold_along_x(map, v),
        Fold::Y(v) => fold_along_y(map, v),
    });
    visualize_map(map);
}

fn main() {
    let input = include_str!("../input.txt");
    let (mut map, folds) = parse_input(input);
    println!("Part 1: {}", solve_part1(&mut map, folds));

    let (mut map, folds) = parse_input(input);
    solve_part2(&mut map, folds);
}

#[test]
fn part1() {
    let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    let (mut map, folds) = parse_input(input);
    assert_eq!(17, solve_part1(&mut map, folds));
}
