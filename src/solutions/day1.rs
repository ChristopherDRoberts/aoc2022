use crate::Solution;

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
    pub(crate) fn parse_input(input: &str) -> Vec<usize> {
        input
            .trim()
            .split("\n\n")
            .map(|s| s.split("\n").map(|x| x.parse::<usize>().unwrap()).sum())
            .collect()
    }

    pub(crate) fn soln_impl(input: &str, n: usize) -> usize {
        let parsed_input = Day1::parse_input(input);
        let mut food = parsed_input;
        food.sort();
        food.iter().rev().take(n).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
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
    fn part2_ex1() {
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