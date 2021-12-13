use std::collections::{HashMap, HashSet};

fn is_lowercase(s: &str) -> bool {
    s.chars().all(|c| c.is_lowercase())
}

fn can_visit(visited: &HashMap<&str, usize>, next: &str, max_visit: usize) -> bool {
    let visit_flag = visited.values().any(|&v| v >= max_visit);
    let next_count = visited.get(next).unwrap_or(&0);
    match visit_flag {
        true => return *next_count < 1,
        false => return *next_count < max_visit,
    }
}

pub struct Graph {
    graph: HashMap<String, Vec<(String, bool)>>,
}
impl Graph {
    pub fn new() -> Graph {
        Graph {
            graph: HashMap::new(),
        }
    }

    pub fn from_tuples(edges: Vec<(&str, &str)>) -> Graph {
        let mut graph = Self::new();
        for (u, v) in edges {
            graph.add_edge(u, v);
        }
        graph
    }

    pub fn add_edge(&mut self, u: &str, v: &str) {
        let node = self.graph.entry(u.to_string()).or_insert(Vec::new());
        node.push((v.to_string(), is_lowercase(v)));

        let node = self.graph.entry(v.to_string()).or_insert(Vec::new());
        node.push((u.to_string(), is_lowercase(u)));
    }

    pub fn count_all_paths<'a>(
        &'a self,
        start: &'a str,
        end: &'a str,
        visited: &mut HashMap<&'a str, usize>,
        max_visit: usize,
    ) -> usize {
        self.graph
            .get(start)
            .unwrap()
            .iter()
            .fold(0, |acc, (v, lc)| match v {
                next if next == end => acc + 1,
                next if next.as_str() == "start" => acc,
                next => {
                    if *lc && !can_visit(&visited.clone(), next.as_str(), max_visit) {
                        return acc;
                    }

                    if *lc {
                        let count = visited.entry(next.as_str()).or_insert(0);
                        *count += 1;
                    }
                    let paths = self.count_all_paths(next.as_str(), end, visited, max_visit);
                    let count = visited.entry(next.as_str()).or_insert(0);
                    if *count >= 1 {
                        *count -= 1;
                    }
                    acc + paths
                }
            })
    }
}

fn solve_part1(input: &str) -> usize {
    let edges: Vec<(&str, &str)> = input
        .split('\n')
        .filter_map(|s| s.split_once('-'))
        .collect();
    let graph = Graph::from_tuples(edges);
    let mut visited = HashMap::new();
    graph.count_all_paths("start", "end", &mut visited, 1)
}

fn solve_part2(input: &str) -> usize {
    let edges: Vec<(&str, &str)> = input
        .split('\n')
        .filter_map(|s| s.split_once('-'))
        .collect();
    let graph = Graph::from_tuples(edges);
    let mut visited = HashMap::new();
    graph.count_all_paths("start", "end", &mut visited, 2)
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", solve_part1(input));

    let input = include_str!("../input.txt");
    println!("Part 2: {}", solve_part2(input));
}

#[test]
fn part1() {
    let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
    assert_eq!(226, solve_part1(input));
}

#[test]
fn part2() {
    let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
    assert_eq!(3509, solve_part2(input));
}
