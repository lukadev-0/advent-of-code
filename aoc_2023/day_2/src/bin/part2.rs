use std::cmp::max;

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", part2(input));
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, list_str) = line.split_once(": ").unwrap();

            let mut red_cubes = 0;
            let mut green_cubes = 0;
            let mut blue_cubes = 0;

            list_str.split("; ").for_each(|subset| {
                subset.split(", ").for_each(|cube_str| {
                    let (amount, color) = cube_str.split_once(" ").unwrap();
                    let amount = amount.parse::<u32>().unwrap();

                    match color {
                        "red" => red_cubes = max(red_cubes, amount),
                        "green" => green_cubes = max(green_cubes, amount),
                        "blue" => blue_cubes = max(blue_cubes, amount),
                        _ => unreachable!(),
                    }
                })
            });

            red_cubes * green_cubes * blue_cubes
        })
        .sum()
}
