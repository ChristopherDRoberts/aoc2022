use crate::Solution;

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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1_ex1() {
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
    fn part2_ex1() {
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
}
