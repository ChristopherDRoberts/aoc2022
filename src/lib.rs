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

#[cfg(test)]
mod tests {
    use std::result;

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
}
