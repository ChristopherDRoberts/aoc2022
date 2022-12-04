use std::fs;

pub trait Solution {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

pub fn read_input(input_dir: &str, day: usize) -> String {
    let file_path = format!("{input_dir}/day{day}.txt");
    fs::read_to_string(file_path).unwrap()
}

pub struct Day1;

impl Solution for Day1 {
    fn part1(&self, input: &str) -> String {
        format!("{}", Day1::soln_impl(input, 1))
    }

    fn part2(&self, input: &str) -> String {
        format!("{}", Day1::soln_impl(input, 3))
    }
}

impl Day1 {
    fn parse_input(input: &str) -> Vec<usize> {
        input
            .trim()
            .split("\n\n")
            .map(|s| s.split("\n").map(|x| x.parse::<usize>().unwrap()).sum())
            .collect()
    }

    fn soln_impl(input: &str, n: usize) -> usize {
        let parsed_input = Day1::parse_input(input);
        let mut food = parsed_input;
        food.sort();
        food.iter().rev().take(n).sum()
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

pub struct Day2;

impl Solution for Day2 {
    fn part1(&self, input: &str) -> String {
        let parsed_input = Day2::parse_input_part1(input);
        let score: usize = parsed_input
            .iter()
            .map(|round| Day2::score_round(round))
            .sum();
        format!("{score}")
    }

    fn part2(&self, input: &str) -> String {
        let parsed_input = Day2::parse_input_part2(input);
        let score: usize = parsed_input
            .iter()
            .map(|round| Day2::score_round(round))
            .sum();
        format!("{score}")
    }
}

impl Day2 {
    fn parse_input_part1(input: &str) -> Vec<Vec<Shape>> {
        input
            .trim_end()
            .split('\n')
            .map(|l| {
                l.split(' ')
                    .map(|s| match s {
                        "A" | "X" => Shape::Rock,
                        "B" | "Y" => Shape::Paper,
                        "C" | "Z" => Shape::Scissors,
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect()
    }

    fn parse_input_part2(input: &str) -> Vec<Vec<Shape>> {
        let split_input = input.trim_end().split('\n');
        let mut rounds = Vec::new();
        for row in split_input {
            let round = match row {
                "A X" => vec![Shape::Rock, Shape::Scissors],
                "A Y" => vec![Shape::Rock, Shape::Rock],
                "A Z" => vec![Shape::Rock, Shape::Paper],

                "B X" => vec![Shape::Paper, Shape::Rock],
                "B Y" => vec![Shape::Paper, Shape::Paper],
                "B Z" => vec![Shape::Paper, Shape::Scissors],

                "C X" => vec![Shape::Scissors, Shape::Paper],
                "C Y" => vec![Shape::Scissors, Shape::Scissors],
                "C Z" => vec![Shape::Scissors, Shape::Rock],

                _ => panic!(),
            };
            rounds.push(round);
        }
        return rounds;
    }

    fn score_round(round: &Vec<Shape>) -> usize {
        let shape_score = match round[1] {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        let round_score = match round[..] {
            [play, counter] if play == counter => 3,
            [play, counter] if play == Shape::Rock && counter == Shape::Paper => 6,
            [play, counter] if play == Shape::Paper && counter == Shape::Scissors => 6,
            [play, counter] if play == Shape::Scissors && counter == Shape::Rock => 6,
            _ => 0,
        };

        shape_score + round_score
    }
}

use std::collections::HashSet;
pub struct Day3;

impl Solution for Day3 {
    fn part1(&self, input: &str) -> String {
        let parsed_input = Day3::parse_input(input);
        let result: usize = parsed_input
            .iter()
            .map(|x| Day3::common_item(x))
            .map(|i| Day3::priority(i))
            .sum();
        format!("{result}")
    }

    fn part2(&self, input: &str) -> String {
        let parsed_input = Day3::parse_input(input);
        let mut result = 0;
        for i in (0..parsed_input.len()).step_by(3) {
            let mut group = Vec::new();
            for j in 0..3 {
                group.push(&parsed_input[i + j]);
            }
            let badge = Day3::find_badge(&group);
            result += Day3::priority(badge);
        }
        format!("{result}")
    }
}

impl Day3 {
    fn parse_input(input: &str) -> Vec<Vec<char>> {
        input
            .trim_end()
            .split('\n')
            .map(|s| s.chars().collect())
            .collect()
    }

    fn priority(item: char) -> usize {
        if item.is_ascii_lowercase() {
            let x = (item as u32) - 96;
            x as usize
        } else {
            let x = (item as u32) - 38;
            x as usize
        }
    }

    fn common_item(rucksack: &Vec<char>) -> char {
        let mid = rucksack.len() / 2;
        let left: HashSet<&char> = rucksack[0..mid].iter().collect();
        let right: HashSet<&char> = rucksack[mid..].iter().collect();
        let mut intersection = left.intersection(&right);
        **intersection.next().unwrap()
    }

    fn find_badge(group: &Vec<&Vec<char>>) -> char {
        let first: HashSet<&char> = group[0].iter().collect();
        let second: HashSet<&char> = group[1].iter().collect();
        let third: HashSet<&char> = group[2].iter().collect();

        let x: HashSet<&char> = first.intersection(&second).map(|c| *c).collect();
        let y: HashSet<&char> = third.intersection(&x).map(|c| *c).collect();
        **y.iter().next().unwrap()
    }
}

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
    fn day1_part1_ex1() {
        let soln = Day1 {};
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
        let result = soln.part1(input);
        assert_eq!(result, "24000");
    }

    #[test]
    fn day1_part2_ex1() {
        let soln = Day1 {};
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
        let result = soln.part2(input);
        assert_eq!(result, "45000");
    }

    #[test]
    fn day2_part1_ex1() {
        let soln = Day2 {};
        let input = "A Y
B X
C Z
";
        let result = soln.part1(input);
        assert_eq!(result, "15")
    }

    #[test]
    fn day2_part2_ex1() {
        let soln = Day2 {};
        let input = "A Y
B X
C Z
";
        let result = soln.part2(input);
        assert_eq!(result, "12")
    }

    #[test]
    fn day3_part1_ex1() {
        let soln = Day3 {};
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw        
";
        let result = soln.part1(input);
        assert_eq!(result, "157")
    }

    #[test]
    fn day3_part2_ex1() {
        let soln = Day3 {};
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw        
";
        let result = soln.part2(input);
        assert_eq!(result, "70")
    }

    #[test]
    fn day4_part1_ex1() {
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
    fn day4_part2_ex1() {
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
