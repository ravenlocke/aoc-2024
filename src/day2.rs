struct InputParser<'a> {
    content: &'a str,
    idx: usize,
}

impl InputParser<'_> {
    fn parse_next(&mut self, vec: &mut Vec<i64>) -> bool {
        let mut n = 0i64;

        for idx in self.idx..self.content.len() {
            let byte = self.content.as_bytes()[idx];
            if byte.is_ascii_digit() {
                n *= 10;
                n += (byte - b'0') as i64
            } else if byte == b'\n' {
                vec.push(n);
                self.idx = idx + 1;
                return true;
            } else {
                vec.push(n);
                n = 0
            }
        }
        false
    }

    fn new(content: &str) -> InputParser {
        InputParser { content, idx: 0 }
    }
}

#[inline(always)]
fn is_safe_list_pt1(list: &[i64]) -> bool {
    let mut iterator = list.iter();

    let (mut current, mut next) = (
        iterator.next().expect("could not retrieve first"),
        iterator.next().expect("could not retrieve second"),
    );
    let initial_delta: i64 = current - next;
    let negative_delta = if initial_delta == 0 || initial_delta.abs() > 3 {
        return false;
    } else {
        initial_delta.is_negative()
    };

    for i in iterator {
        (current, next) = (next, i);
        let delta = current - next;
        if (delta == 0) || (delta.is_negative() != negative_delta) || (delta.abs() > 3) {
            return false;
        }
    }
    true
}

#[inline(always)]
fn is_safe_list_pt2(list: &[i64]) -> bool {
    let mut iterator = list.iter().filter(|i| **i != i64::MAX);

    let (mut current, mut next) = (
        iterator.next().expect("could not retrieve first"),
        iterator.next().expect("could not retrieve second"),
    );
    let initial_delta: i64 = current - next;
    let negative_delta = if initial_delta == 0 || initial_delta.abs() > 3 {
        return false;
    } else {
        initial_delta.is_negative()
    };

    for i in iterator {
        (current, next) = (next, i);
        let delta = current - next;
        if (delta == 0) || (delta.is_negative() != negative_delta) || (delta.abs() > 3) {
            return false;
        }
    }
    true
}


pub fn part1(contents: &str) -> usize {
    let mut count = 0usize;
    let mut list = vec![];
    let mut parser = InputParser::new(contents);

    while parser.parse_next(&mut list) {
        if is_safe_list_pt1(&list) {
            count += 1
        }
        list.clear();
    }
    count
}


pub fn part2(contents: &str) -> usize {
    let mut count = 0usize;
    let mut list = vec![];
    let mut parser = InputParser::new(contents);

    while parser.parse_next(&mut list) {
        if (0..list.len()).any(|idx| {
            let removed = list[idx];
            list[idx] = i64::MAX;
            let to_return = is_safe_list_pt2(&list);
            list[idx] = removed;
            to_return
        }) {
            count += 1
        }
        list.clear();
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_solution() {
        let contents = include_str!("../inputs/day02.txt");
        let result = part1(&contents);
        assert_eq!(result, 432);
    }

    #[test]
    fn test_part_two_solution() {
        let contents = include_str!("../inputs/day02.txt");
        let result = part2(contents);
        assert_eq!(result, 488);
    }
}
