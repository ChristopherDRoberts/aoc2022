use crate::Solution;
use regex::Regex;
struct Move {
    count: u32,
    source: u32,
    destination: u32,
}

pub struct Day5;

impl Solution for Day5 {
    fn part1(&self, input: &str) -> String {
        let (mut stacks, moves) = Day5::parse_input(input);
        for mov in moves {
            for _ in 0..mov.count {
                let item = stacks[(mov.source - 1) as usize].pop().unwrap();
                stacks[(mov.destination - 1) as usize].push(item);
            }
        }
        let mut result = String::new();
        for mut stack in stacks {
            result.push(stack.pop().unwrap());
        }
        result
    }

    fn part2(&self, input: &str) -> String {
        let (mut stacks, moves) = Day5::parse_input(input);
        let mut temp_stack = Vec::new();
        for mov in moves {
            for _ in 0..mov.count {
                temp_stack.push(stacks[(mov.source - 1) as usize].pop().unwrap());
            }
            for _ in 0..mov.count {
                stacks[(mov.destination - 1) as usize].push(temp_stack.pop().unwrap());
            }
        }
        let mut result = String::new();
        for mut stack in stacks {
            result.push(stack.pop().unwrap());
        }
        result
    }
}

impl Day5 {
    fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
        let parts: Vec<&str> = input.trim().split("\n\n").collect();
        let stacks = parts[0].split('\n').map(|l| l.chars().collect()).collect();
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let moves = parts[1]
            .split('\n')
            .map(|l| {
                let caps = re.captures(l).unwrap();
                Move {
                    count: caps[1].parse().unwrap(),
                    source: caps[2].parse().unwrap(),
                    destination: caps[3].parse().unwrap(),
                }
            })
            .collect();
        (stacks, moves)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        let soln = Day5 {};
        let input = "ZN
MCD
P

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
        let result = soln.part1(input);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part1_ex2() {
        let soln = Day5 {};
        let input = "ZN
MCD
P

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
        let result = soln.part2(input);
        assert_eq!(result, "MCD");
    }
}
