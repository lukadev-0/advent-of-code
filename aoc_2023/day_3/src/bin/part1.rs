fn main() {
    let input = include_str!("./input.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> u32 {
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;

    lines.iter().enumerate().for_each(|(line_idx, line)| {
        let mut iter = line.iter().enumerate().peekable();

        while let Some((start, _)) = iter.find(|(_, char)| char.is_ascii_digit()) {
            let end = iter
                .find(|(_, char)| !char.is_ascii_digit())
                .map_or(line.len(), |(idx, _)| idx);

            let has_adjacent_symbol =
                // Check for symbol along the top
                (line_idx > 0 && (start.saturating_sub(1)..=end.min(line.len()-1))
                    .map(|idx| &lines[line_idx - 1][idx])
                    .any(is_symbol))

                // Check for symbol along the bottom
                || (line_idx < lines.len() - 1 && (start.saturating_sub(1)..=end.min(line.len()-1))
                    .map(|idx| &lines[line_idx + 1][idx])
                    .any(is_symbol))

                // Check for symbol on the left
                || (start > 0 && is_symbol(&lines[line_idx][start - 1]))

                // Check for symbol on the right
                || (end < line.len() && is_symbol(&lines[line_idx][end]));

            if has_adjacent_symbol {
                sum += line[start..end]
                    .iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
            }
        }
    });

    sum
}

fn is_symbol(c: &char) -> bool {
    *c != '.' && c.is_ascii_punctuation()
}
