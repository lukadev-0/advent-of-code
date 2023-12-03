use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", part2(input));
}

fn part2(input: &str) -> u32 {
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut gears = HashMap::new();

    lines.iter().enumerate().for_each(|(line_idx, line)| {
        let mut iter = line.iter().enumerate().peekable();

        while let Some((start, _)) = iter.find(|(_, char)| char.is_ascii_digit()) {
            let end = iter
                .find(|(_, char)| !char.is_ascii_digit())
                .map_or(line.len(), |(idx, _)| idx);

            let has_adjacent_symbol =
                // Check for gear along the top
                (if line_idx > 0 {
                    (start.saturating_sub(1)..=end.min(line.len()-1))
                        .find_map(|idx| if lines[line_idx - 1][idx] == '*' { Some((line_idx - 1, idx)) } else { None })
                } else {
                    None
                })

                // Check for gear along the bottom
                .or_else(|| if line_idx < lines.len() - 1 {
                    (start.saturating_sub(1)..=end.min(line.len()-1))
                        .find_map(|idx| if lines[line_idx + 1][idx] == '*' { Some((line_idx + 1, idx)) } else { None })
                } else {
                        None
                })

                // Check for gear on the left
                .or_else(|| if start > 0 {
                    if lines[line_idx][start - 1] == '*' { Some((line_idx, start - 1)) } else { None }
                } else {
                    None
                })

                // Check for gear on the right
                .or_else(|| if end < line.len() {
                    if lines[line_idx][end] == '*' { Some((line_idx, end)) } else { None }
                } else {
                    None
                });

            if let Some(pos) = has_adjacent_symbol {
                let num = line[start..end].iter().collect::<String>().parse::<u32>().unwrap();

                gears.entry(pos).and_modify(|x| {
                    *x = match *x {
                        (false, Some(a), None) => (true, Some(a), Some(num)),
                        _ => (false, None, None),
                    };
                }).or_insert((false, Some(num), None));
            }
        }
    });

    gears
        .values()
        .filter(|x| x.0)
        .map(|x| x.1.unwrap() * x.2.unwrap())
        .sum()
}
