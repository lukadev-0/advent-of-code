use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, card_nums) = line.split_once(": ").unwrap();

            let (winning_nums, my_nums) = card_nums.split_once(" | ").unwrap();

            let set = winning_nums.split_whitespace().collect::<HashSet<&str>>();

            let mut score = 0;

            my_nums
                .split_whitespace()
                .filter(|x| set.contains(x))
                .for_each(|_| {
                    if score == 0 {
                        score = 1;
                    } else {
                        score *= 2;
                    }
                });

            score
        })
        .sum()
}
