use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", part2(input));
}

// this is such a slow solution what am i doing lmao
fn part2(input: &str) -> u32 {
    let mut card_instances = HashMap::new();

    let lines = input.lines().collect::<Vec<_>>();

    let mut queue = lines
        .iter()
        .map(|line| {
            let (card_str, _) = line.split_once(": ").unwrap();
            let card = card_str[5..].trim().parse::<u32>().unwrap();

            card
        })
        .collect::<Vec<_>>();

    while let Some(card) = queue.pop() {
        card_instances
            .entry(card)
            .and_modify(|num| *num += 1)
            .or_insert(1);

        let line = lines[(card - 1) as usize];
        let (_, card_nums) = line.split_once(": ").unwrap();
        let matching_cards = get_matching_cards(card_nums);

        (1..=matching_cards).for_each(|num| queue.push(card + num as u32));
        println!("{}", queue.len());
    }

    card_instances.values().sum()
}

fn get_matching_cards(card_nums: &str) -> usize {
    let (winning_nums, my_nums) = card_nums.split_once(" | ").unwrap();

    let set = winning_nums.split_whitespace().collect::<HashSet<&str>>();

    my_nums
        .split_whitespace()
        .filter(|x| set.contains(x))
        .count()
}
