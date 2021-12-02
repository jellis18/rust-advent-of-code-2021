fn get_product_of_headings(headings: Vec<(&str, i32)>) -> i32 {
    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;
    for (direction, value) in headings {
        match direction {
            "forward" => {
                distance += value;
                depth += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => panic!("Ya done messed up"),
        }
    }
    depth * distance
}

fn main() {
    let headings: Vec<(&str, i32)> = include_str!("../input.txt")
        .lines()
        .map(|line| line.split(' ').collect::<Vec<&str>>())
        .map(|v| (v[0], v[1].parse::<i32>().unwrap()))
        .collect();

    println!(
        "Product of direction and depth is {}",
        get_product_of_headings(headings)
    )
}

#[test]
fn test_product_of_headings() {
    let input = vec![
        ("forward", 5),
        ("down", 5),
        ("forward", 8),
        ("up", 3),
        ("down", 8),
        ("forward", 2),
    ];

    assert_eq!(900, get_product_of_headings(input))
}
