fn main() {
    let input = include_str!("./input.txt");
    println!("{}", part_1(input));
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter(|char| char.is_numeric()).peekable();

            format!(
                "{}{}",
                digits.peek().unwrap().clone(),
                digits.next_back().unwrap()
            )
            .parse::<u32>()
            .unwrap()
        })
        .sum()
}
