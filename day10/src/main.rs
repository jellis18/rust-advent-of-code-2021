#[derive(PartialEq, Debug)]
enum Completion {
    ParseError(usize),
    Incomplete(usize),
}

fn get_line_score(line: &str) -> Completion {
    let mut stack = Vec::new();
    // pop off elements in LIFO order
    for c in line.chars() {
        match c {
            '(' | '{' | '<' | '[' => stack.push(c),
            _ => {
                let open = stack.pop().unwrap();
                match c {
                    ')' if open != '(' => return Completion::ParseError(3),
                    ']' if open != '[' => return Completion::ParseError(57),
                    '}' if open != '{' => return Completion::ParseError(1197),
                    '>' if open != '<' => return Completion::ParseError(25137),
                    _ => {}
                }
            }
        }
    }
    Completion::Incomplete(get_completion_score(stack))
}

fn get_completion_score(stack: Vec<char>) -> usize {
    let mut score = 0;
    for char in stack.iter().rev() {
        match char {
            '(' => score = score * 5 + 1,
            '[' => score = score * 5 + 2,
            '{' => score = score * 5 + 3,
            '<' => score = score * 5 + 4,
            _ => unreachable!(),
        }
    }
    score
}

fn solve_part1(input: &str) -> usize {
    input
        .split('\n')
        .filter_map(|line| match get_line_score(line) {
            Completion::ParseError(v) => Some(v),
            _ => None,
        })
        .sum::<usize>()
}

fn solve_part2(input: &str) -> usize {
    let mut scores: Vec<usize> = input
        .split('\n')
        .filter_map(|line| match get_line_score(line) {
            Completion::Incomplete(v) => Some(v),
            _ => None,
        })
        .collect();

    scores.sort();
    let middle_index = scores.len() / 2;
    scores[middle_index]
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part1 {:?}", solve_part1(input));
    println!("Part2 {:?}", solve_part2(input));
}

#[test]
fn test_get_line_score() {
    assert_eq!(
        get_line_score("{([(<{}[<>[]}>{[]{[(<()>"),
        Completion::ParseError(1197)
    );
    assert_eq!(
        get_line_score("[[<[([]))<([[{}[[()]]]"),
        Completion::ParseError(3)
    );
    assert_eq!(
        get_line_score("[{[{({}]{}}([{[{{{}}([]"),
        Completion::ParseError(57)
    );
    assert_eq!(
        get_line_score("[<(<(<(<{}))><([]([]()"),
        Completion::ParseError(3)
    );
    assert_eq!(
        get_line_score("<{([([[(<>()){}]>(<<{{"),
        Completion::ParseError(25137)
    );
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
    assert_eq!(solve_part1(input), 26397)
}

#[test]
fn test_completion_score() {
    let stack = vec!['<', '{', '(', '['];
    assert_eq!(get_completion_score(stack), 294);
}

#[test]
fn test_part2() {
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
    assert_eq!(solve_part2(input), 288957)
}
