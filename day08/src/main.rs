fn parse_input(input: &str) -> Vec<Vec<&str>> {
    let x: Vec<Vec<&str>> = input
        .split('\n')
        .filter_map(|s| {
            s.trim()
                .split_once('|')
                .map(|split| split.1.trim())
                .map(|s| s.split_whitespace().collect::<Vec<_>>())
        })
        .collect();
    x
}

fn parse_input_part2(input: &str) -> (Vec<Vec<&str>>, Vec<Vec<&str>>) {
    let patterns: Vec<Vec<&str>> = input
        .split('\n')
        .filter_map(|s| {
            s.trim()
                .split_once('|')
                .map(|split| split.0.trim())
                .map(|s| s.split_whitespace().collect::<Vec<_>>())
        })
        .collect();

    let samples: Vec<Vec<&str>> = input
        .split('\n')
        .filter_map(|s| {
            s.trim()
                .split_once('|')
                .map(|split| split.1.trim())
                .map(|s| s.split_whitespace().collect::<Vec<_>>())
        })
        .collect();
    (patterns, samples)
}

fn solve_part1(vecs: Vec<Vec<&str>>) -> usize {
    let unique_segment_lengths = [2, 3, 4, 7];
    let mut count = 0;
    for v in &vecs {
        count += v
            .iter()
            .filter(|e| unique_segment_lengths.contains(&e.len()))
            .count();
    }
    count
}

fn find_by_length<'a>(sample: &'a Vec<&str>, len: usize) -> &'a str {
    sample.iter().find(|e| e.len() == len).unwrap()
}

fn decode(pattern: &str, one: &str, four: &str) -> u32 {
    // first match on the known lengths
    match pattern.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        7 => 8,
        // then match on the length, and the number of intersectins with the 1 and 4 shapes
        len => match (
            len,
            pattern.chars().filter(|&c| one.contains(c)).count(),
            pattern.chars().filter(|&c| four.contains(c)).count(),
        ) {
            (5, 1, 3) => 5,
            (5, 2, 3) => 3,
            (5, _, 2) => 2,
            (6, 1, _) => 6,
            (6, _, 3) => 0,
            (6, _, 4) => 9,
            _ => unreachable!(),
        },
    }
}

fn solve_part2(patterns: Vec<Vec<&str>>, samples: Vec<Vec<&str>>) -> u32 {
    let mut sum = 0;
    for (v, s) in patterns.iter().zip(samples.iter()) {
        let one = find_by_length(v, 2);
        let four = find_by_length(v, 4);
        sum += s.iter().enumerate().fold(0, |acc, (i, p)| {
            acc + decode(p, one, four) * 10_u32.pow(3 - i as u32)
        });
    }
    sum
}

fn main() {
    let input = include_str!("../input.txt");
    let x = parse_input(input);
    println!("Part 1: {:?}", solve_part1(x));

    let (patterns, samples) = parse_input_part2(input);
    println!("Part 2: {:?}", solve_part2(patterns, samples));
}

#[test]
fn part1() {
    let input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";
    let x = parse_input(input);
    assert_eq!(26, solve_part1(x));
}

#[test]
fn part2() {
    let input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";
    let (patterns, samples) = parse_input_part2(input);
    assert_eq!(61229, solve_part2(patterns, samples));
}
