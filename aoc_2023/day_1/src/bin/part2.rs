use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", part_2(input));
}

fn part_2(input: &str) -> u32 {
    let re = Regex::new("[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();

    input
        .lines()
        .map(|line| {
            let mut digits = re
                .captures_iter(line)
                .map(|capture| match &capture[0] {
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    capture => capture.parse::<u32>().unwrap(),
                })
                .peekable();

            format!(
                "{}{}",
                digits.peek().unwrap().clone(),
                digits.last().unwrap()
            )
            .parse::<u32>()
            .unwrap()
        })
        .sum()
}
