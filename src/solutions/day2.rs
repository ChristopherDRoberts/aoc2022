use crate::Solution;

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
    use super::*;

    #[test]
    fn part1_ex1() {
        let soln = Day2 {};
        let input = "A Y
B X
C Z
";
        let result = soln.part1(input);
        assert_eq!(result, "15")
    }

    #[test]
    fn part2_ex1() {
        let soln = Day2 {};
        let input = "A Y
B X
C Z
";
        let result = soln.part2(input);
        assert_eq!(result, "12")
    }
}
