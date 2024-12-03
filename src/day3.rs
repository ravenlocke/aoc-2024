use regex::Regex;

fn parse_radix_10(input: &str) -> u64 {
    let mut n = 0;
    for i in input.as_bytes() {
        n *= 10;
        n += *i as u64
    }

    let correction = {
        match input.len() {
            1 => 48,
            2 => 528,
            3 => 5_328,
            4 => 53_328,
            5 => 533_328,
            6 => 5_333_328,
            _ => panic!("unexpected number length to correct for")

        }
    };

    n -= correction;

    return n
}

pub fn part1(content: &str) -> u64 {
    let pattern = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").expect("could not parse regex pattern");
    pattern.captures_iter(content).map(|i| {
        let a = parse_radix_10(&i["a"]);
        let b = parse_radix_10(&i["b"]);
        a * b
    }).sum()
}


pub fn part2(content: &str) -> u64 {
    let pattern = Regex::new(r"(mul\((?<a>\d+),(?<b>\d+)\)|(?<dis>don\'t\(\))|(?<en>do\(\)))").expect("could not parse regex pattern");
    let mut matching = true;

    pattern.captures_iter(content).map(|i| {
        if let Some(_) = i.name("dis") {
            matching = false;
            return 0
        } else if let Some(_) = i.name("en") {
            matching = true;
            return 0
        } else if matching {
            let a = parse_radix_10(&i["a"]);
            let b = parse_radix_10(&i["b"]);
            return a * b
        } else {
            return 0
        }
    }).sum()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let contents = include_str!("../inputs/day03.txt");
        let result = part1(&contents);
        assert_eq!(result, 182_780_583);
    }

    #[test]
    fn test_part_two_solution() {
        let contents = include_str!("../inputs/day03.txt");
        let result = part2(contents);
        assert_eq!(result, 90_772_405);
    }
}