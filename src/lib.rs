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
        let parsed_input = Day1::parse_input(input);
        let food = parsed_input.iter().max().unwrap();
        format!("{food}")
    }

    fn part2(&self, input: &str) -> String {
        let parsed_input = Day1::parse_input(input);
        let mut food = parsed_input;
        food.sort();
        let sum: usize = food.iter().rev().take(3).sum();
        format!("{sum}")
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
}
