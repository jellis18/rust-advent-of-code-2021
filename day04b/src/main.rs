use std::collections::HashMap;

struct BingoBoard {
    map: HashMap<usize, Vec<(usize, usize)>>,
    reverse_map: HashMap<(usize, usize), usize>,
    marked_values: Vec<usize>,
}

impl BingoBoard {
    fn from_string(s: &str) -> BingoBoard {
        let mut map = HashMap::new();
        for (ii, line) in s.trim().split("\n").enumerate() {
            for (jj, val) in line.split_whitespace().enumerate() {
                map.entry(val.parse::<usize>().unwrap())
                    .or_insert(Vec::new())
                    .push((ii, jj));
            }
        }
        let mut reverse_map = HashMap::new();
        for (val, rowcol) in map.clone() {
            for (row, col) in rowcol {
                reverse_map.insert((row, col), val);
            }
        }

        BingoBoard {
            map,
            reverse_map,
            marked_values: Vec::new(),
        }
    }

    fn update(&mut self, value: usize) {
        if self.map.contains_key(&value) {
            self.marked_values.push(value)
        }
    }

    fn is_winner(&self) -> bool {
        // loop over rows and see if all are matched
        for ii in 0..5 {
            let x = self
                .reverse_map
                .iter()
                .filter(|((row, _), v)| (row == &ii) & self.marked_values.contains(v))
                .count();
            if x == 5 {
                return true;
            }
        }

        // loop over cols and see if all are matched
        for ii in 0..5 {
            let x = self
                .reverse_map
                .iter()
                .filter(|((_, col), v)| (col == &ii) & self.marked_values.contains(v))
                .count();
            if x == 5 {
                return true;
            }
        }
        false
    }

    fn score(&self) -> usize {
        let last_called = self.marked_values.last().unwrap();
        let sum_unmarked: usize = self
            .reverse_map
            .values()
            .filter(|v| !self.marked_values.contains(v))
            .sum();
        last_called * sum_unmarked
    }
}

fn main() {
    let input = include_str!("../input.txt");

    // first line are drawn numbers
    let draws: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    // build the boards
    let mut boards: Vec<BingoBoard> = input
        .split("\n\n")
        .skip(1)
        .map(|s| BingoBoard::from_string(s))
        .collect();

    // play the game and find score of winning board
    let mut winner_scores = Vec::new();
    let mut winners = Vec::new();
    for v in draws {
        let mut board_count = 0;
        for board in &mut boards {
            if !winners.contains(&board_count) {
                board.update(v);
                if board.is_winner() == true {
                    winner_scores.push(board.score());
                    winners.push(board_count)
                }
            }
            board_count += 1;
        }
    }
    println!(
        "The score of the last winner is {}",
        winner_scores.last().unwrap()
    );
}
