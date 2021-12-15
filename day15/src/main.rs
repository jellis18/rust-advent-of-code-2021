use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

const NEIGHBORS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

type Graph = HashMap<(usize, usize), Vec<Edge>>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra's shortest path
// Modified from here: https://doc.rust-lang.org/std/collections/binary_heap/index.html
fn shortest_path(graph: &Graph, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let mut dist = HashMap::new();
    for key in graph.keys() {
        dist.insert(*key, usize::MAX);
    }

    let mut spt = BinaryHeap::new();

    // start at "start" with 0 cost
    dist.insert(start, 0);
    spt.push(State {
        position: start,
        cost: 0,
    });

    while let Some(State { cost, position }) = spt.pop() {
        // stop if we have reached the end
        if position == end {
            return Some(cost);
        }

        // if we already found a better way, continue
        if cost > dist[&position] {
            continue;
        }

        // for each node in the neighborhood, see if we can find a lower cost path
        for edge in &graph[&position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            if next.cost < dist[&next.position] {
                spt.push(next);
                dist.insert(next.position, next.cost);
            }
        }
    }
    None
}

#[derive(Debug)]
struct Edge {
    node: (usize, usize),
    cost: usize,
}

fn get_cavern_map_part1(input: &str) -> Vec<Vec<u32>> {
    let cavern: Vec<Vec<u32>> = input
        .split('\n')
        .map(|s| s.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .collect();
    cavern
}

fn get_cavern_map_part2(input: &str) -> Vec<Vec<u32>> {
    let cavern: Vec<Vec<u32>> = input
        .split('\n')
        .map(|s| s.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .collect();
    let mut extended_cavern = Vec::new();
    for row in cavern {
        let mut extended_row: Vec<u32> = Vec::new();
        for fac in 0..5 {
            extended_row.extend(
                row.iter()
                    .map(|v| ((v + fac - 1) % 9) + 1)
                    .collect::<Vec<_>>(),
            );
        }
        // add extended rows
        extended_cavern.push(extended_row);
    }
    // now duplicate and advance rows
    let mut doubly_extended_cavern = Vec::new();
    for fac in 0..5 {
        for row in &extended_cavern {
            doubly_extended_cavern.push(
                row.iter()
                    .map(|v| ((v + fac - 1) % 9) + 1)
                    .collect::<Vec<_>>(),
            );
        }
    }
    doubly_extended_cavern
}

fn build_graph(cavern: Vec<Vec<u32>>) -> Graph {
    // form graph
    let mut graph = HashMap::new();
    for i in 0..cavern.len() {
        for j in 0..cavern[0].len() {
            let edges = graph.entry((i, j)).or_insert(Vec::new());
            NEIGHBORS
                .iter()
                .map(|(xx, yy)| ((i as isize + xx) as usize, (j as isize + yy) as usize))
                .for_each(|(x, y)| match cavern.get(x).and_then(|v| v.get(y)) {
                    Some(cost) => edges.push(Edge {
                        node: (x, y),
                        cost: *cost as usize,
                    }),

                    None => {}
                });
        }
    }
    graph
}

fn main() {
    let input = include_str!("../input.txt");
    let cavern = get_cavern_map_part1(input);
    let graph = build_graph(cavern);
    let start = graph.keys().min().unwrap();
    let end = graph.keys().max().unwrap();
    println!("{:?} {:?}", start, end);
    println!("Part 1: {:?}", shortest_path(&graph, *start, *end));

    let cavern = get_cavern_map_part2(input);
    let graph = build_graph(cavern);
    let start = graph.keys().min().unwrap();
    let end = graph.keys().max().unwrap();
    println!("{:?} {:?}", start, end);
    println!("Part 2: {:?}", shortest_path(&graph, *start, *end));
}

#[test]
fn part1() {
    let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    let cavern = get_cavern_map_part1(input);
    let graph = build_graph(cavern);
    let start = (0, 0);
    let end = (9, 9);
    assert_eq!(Some(40), shortest_path(&graph, start, end));
}

#[test]
fn part2() {
    let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    let cavern = get_cavern_map_part2(input);
    let graph = build_graph(cavern);
    let start = *graph.keys().min().unwrap();
    let end = *graph.keys().max().unwrap();
    assert_eq!(Some(315), shortest_path(&graph, start, end));
}
