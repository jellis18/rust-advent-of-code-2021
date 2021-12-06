use std::collections::HashMap;

struct VentMap {
    counter: HashMap<(i32, i32), i32>,
}

impl VentMap {
    fn from_coordinates(coords: Vec<Vec<i32>>) -> VentMap {
        let mut counter = HashMap::new();

        for coord in coords {
            let (x1, y1, x2, y2) = (coord[0], coord[1], coord[2], coord[3]);

            // determine the x and y lengths of the line
            let x_range = x2 - x1;
            let y_range = y2 - y1;

            // determine the step size in each direction
            let x_step = if x_range < 0 {
                -1
            } else if x_range == 0 {
                0
            } else {
                1
            };

            let y_step = if y_range < 0 {
                -1
            } else if y_range == 0 {
                0
            } else {
                1
            };

            // starting in starting position, loop, adding the step to each coordinate
            // until we reach the endpoint
            let (mut x, mut y) = (x1, y1);
            loop {
                let count = counter.entry((x, y)).or_insert(0);
                *count += 1;

                if (x, y) == (x2, y2) {
                    break;
                }

                x += x_step;
                y += y_step
            }
        }

        VentMap { counter }
    }

    fn num_overlap_points(&self, num_overlaps: i32) -> usize {
        self.counter
            .values()
            .filter(|v| v >= &&num_overlaps)
            .count()
    }
}

fn parse_input_part_1(input: &str) -> Vec<Vec<i32>> {
    let coords: Vec<Vec<i32>> = input
        .lines()
        .map(|s| {
            s.split(&['-', ',', '>', ' ', '\n'][..])
                .filter(|s| *s != "")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|v| (v[0] == v[2]) | (v[1] == v[3]))
        .collect();
    coords
}

fn parse_input_part_2(input: &str) -> Vec<Vec<i32>> {
    let coords: Vec<Vec<i32>> = input
        .lines()
        .map(|s| {
            s.split(&['-', ',', '>', ' ', '\n'][..])
                .filter(|s| *s != "")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    coords
}

fn main() {
    // part 1
    let input = include_str!("../input.txt");
    let coords = parse_input_part_1(input);
    let vent_map = VentMap::from_coordinates(coords);
    println!(
        "Number of points with overlap > 2 for part 1: {:?}",
        vent_map.num_overlap_points(2)
    );

    let coords = parse_input_part_2(input);
    let vent_map = VentMap::from_coordinates(coords);
    println!(
        "Number of points with overlap > 2 for part 2: {:?}",
        vent_map.num_overlap_points(2)
    );
}

#[test]
fn part1() {
    let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    let coords = parse_input_part_1(input);
    let vent_map = VentMap::from_coordinates(coords);
    assert_eq!(5, vent_map.num_overlap_points(2));
}

#[test]
fn part2() {
    let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    let coords = parse_input_part_2(input);
    println!("{:?}", coords);
    let vent_map = VentMap::from_coordinates(coords);
    assert_eq!(12, vent_map.num_overlap_points(2));
}
