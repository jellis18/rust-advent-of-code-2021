fn get_line_score(line: &str) -> Option<usize> {
    let mut stack = Vec::new();
    // pop off elements in LIFO order
    for c in line.chars() {
        match c {
            '(' | '{' | '<' | '[' => stack.push(c),
            _ => {
                let open = stack.pop().unwrap();
                match c {
                    ')' if open != '(' => return Some(3),
                    ']' if open != '[' => return Some(57),
                    '}' if open != '{' => return Some(1197),
                    '>' if open != '<' => return Some(25137),
                    _ => {}
                }
            }
        }
    }
    None
}

fn solve_part1(input: &str) -> usize {
    input
        .split('\n')
        .filter_map(|line| get_line_score(line))
        .sum::<usize>()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part1 {:?}", solve_part1(input))
}

#[test]
fn test_get_line_score() {
    assert_eq!(get_line_score("{([(<{}[<>[]}>{[]{[(<()>"), Some(1197));
    assert_eq!(get_line_score("[[<[([]))<([[{}[[()]]]"), Some(3));
    assert_eq!(get_line_score("[{[{({}]{}}([{[{{{}}([]"), Some(57));
    assert_eq!(get_line_score("[<(<(<(<{}))><([]([]()"), Some(3));
    assert_eq!(get_line_score("<{([([[(<>()){}]>(<<{{"), Some(25137));
}

#[test]
fn test_part1() {
    let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    let score: usize = input
        .split('\n')
        .filter_map(|line| get_line_score(line))
        .sum();
    assert_eq!(score, 26397)
}
