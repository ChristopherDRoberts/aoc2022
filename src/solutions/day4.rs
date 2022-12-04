use crate::Solution;

use regex::Regex;
pub struct Day4;

struct Interval {
    left: usize,
    right: usize,
}

impl Interval {
    fn new(left: usize, right: usize) -> Self {
        Self { left, right }
    }

    fn contains(&self, other: &Interval) -> bool {
        self.left <= other.left && self.right >= other.right
    }

    fn overlaps(&self, other: &Interval) -> bool {
        if self.right < other.left || self.left > other.right {
            false
        } else {
            true
        }
    }
}

impl Solution for Day4 {
    fn part1(&self, input: &str) -> String {
        let intervals = Day4::parse_input(input);
        let result = intervals
            .iter()
            .filter(|p| p.0.contains(&p.1) || p.1.contains(&p.0))
            .count();
        format!("{result}")
    }

    fn part2(&self, input: &str) -> String {
        let intervals = Day4::parse_input(input);
        let result = intervals.iter().filter(|p| p.0.overlaps(&p.1)).count();
        format!("{result}")
    }
}

impl Day4 {
    fn parse_input(input: &str) -> Vec<(Interval, Interval)> {
        let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
        input
            .trim()
            .split('\n')
            .map(|l| {
                let caps = re.captures(l).unwrap();
                let first =
                    Interval::new(*(&caps[1].parse().unwrap()), *(&caps[2].parse().unwrap()));
                let second =
                    Interval::new(*(&caps[3].parse().unwrap()), *(&caps[4].parse().unwrap()));
                (first, second)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_ex1() {
        let soln = Day4 {};
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8                
";
        let result = soln.part1(input);
        assert_eq!(result, "2")
    }

    #[test]
    fn part2_ex1() {
        let soln = Day4 {};
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8                
";
        let result = soln.part2(input);
        assert_eq!(result, "4")
    }
}