use aoc2023::read_input;

fn main() {
    let inp = read_input("./inputs/01.txt");
    let part1_sum = inp.lines()
        .map(|line| from_numerics(line))
        .sum::<u32>();

    let part2_sum = inp.lines()
        .map(|line| from_digits(line))
        .sum::<u32>();

    println!("{}", part1_sum);
    println!("{}", part2_sum);
}

fn from_digits(line: &str) -> u32 {
    let digit_names = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut first = None;
    let mut last = 0;

    let mut assign = |d: u32| {
        if first.is_none() {
            first = Some(d);
        }
        last = d;
    };

    for (i, c1) in line.chars().enumerate() {
        if let Some(digit) = c1.to_digit(10) {
            assign(digit);
            continue;
        }
        for (j, digit_name) in digit_names.iter().enumerate() {
            // starts with uses regexes under the hood, not as efficient as it could be
            if line[i..].starts_with(digit_name) {
                let digit = j + 1;
                assign(digit as u32);
            }
        }
    }
    first.unwrap_or_default() * 10 + last
}


fn from_numerics(line: &str) -> u32 {
    let first = line.chars()
        .find_map(|c| c.to_digit(10))
        .unwrap_or_default();

    let last = line.chars()
        .rev()
        .find_map(|c| c.to_digit(10))
        .unwrap_or_default();

    first * 10 + last
}