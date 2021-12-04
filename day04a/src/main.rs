use std::collections::HashMap;
use std::fs;

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
    'outer: for v in draws {
        for board in &mut boards {
            board.update(v);
            if board.is_winner() == true {
                println!("We have a winner with score {}", board.score());
                break 'outer;
            }
        }
    }
}

#[test]
fn test_board_map_is_updated_correctly_with_value_on_board() {
    let board_str = "
22 13 17 11  0
 8  2 23  4  3
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
 ";
    let mut board = BingoBoard::from_string(board_str);
    board.update(3);
    assert_eq!(board.marked_values, vec![3])
}

#[test]
fn test_board_map_is_updated_correctly_with_value_not_on_board() {
    let board_str = "
22 13 17 11  0
 8  2 23  4  3
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
 ";
    let mut board = BingoBoard::from_string(board_str);
    board.update(100);
    assert_eq!(board.marked_values, vec![])
}

#[test]
fn test_winning_board() {
    let board_str = "
22 13 17 11  0
 8  2 23  4  3
21  9 14 16  7
 6 10  3 18  5
 1 2 2 1 1
 ";
    let mut board = BingoBoard::from_string(board_str);
    board.update(1);
    board.update(2);
    assert_eq!(true, board.is_winner());
}

#[test]
fn test_not_winning_board() {
    let board_str = "
22 13 17 11  0
 8  2 23  4  3
21  9 14 16  7
 6 10  3 18  5
 1 2 2 1 1
 ";
    let mut board = BingoBoard::from_string(board_str);
    board.update(1);
    board.update(24);
    assert_eq!(false, board.is_winner());
}
