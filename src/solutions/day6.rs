use crate::Solution;

pub struct Day6;

impl Solution for Day6 {
    fn part1(&self, input: &str) -> String {
        Day6::soln(input, 4)
    }

    fn part2(&self, input: &str) -> String {
        Day6::soln(input, 14)
    }
}

impl Day6 {
    fn parse_input(input: &str) -> Vec<char> {
        input.trim().chars().collect()
    }

    fn distinct(chars: &[char]) -> bool {
        for i in 0..(chars.len() - 1) {
            for j in i + 1..chars.len() {
                if chars[i] == chars[j] {
                    return false;
                }
            }
        }
        true
    }

    fn soln(input: &str, len: usize) -> String {
        let parsed_input = Day6::parse_input(input);
        for i in 0..parsed_input.len() {
            if Day6::distinct(&parsed_input[i..i + len]) {
                return format!("{}", i + len);
            }
        }
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        let soln = Day6 {};
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = soln.part1(input);
        assert_eq!(result, "7");
    }

    #[test]
    fn part2_ex1() {
        let soln = Day6 {};
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = soln.part2(input);
        assert_eq!(result, "19");
    }
}
