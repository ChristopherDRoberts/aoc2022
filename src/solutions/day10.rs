use crate::Solution;

pub struct Day10;

enum Instruction {
    Noop,
    Addx(i32),
}

impl Solution for Day10 {
    fn part1(&self, input: &str) -> String {
        let instructions = Day10::parse(input);
        let x = Day10::run(&instructions);
        let mut result = 0;
        for i in (19..x.len()).step_by(40) {
            result += ((i + 1) as i32) * x[i];
        }
        format!("{result}")
    }

    fn part2(&self, input: &str) -> String {
        let instructions = Day10::parse(input);
        let x = Day10::run(&instructions);
        let mut result = String::new();
        for i in 0..x.len() - 1 {
            if (x[i] - ((i % 40) as i32)).abs() <= 1 {
                result.push('#');
            } else {
                result.push(' ');
            }
            if (i + 1) % 40 == 0 {
                result.push('\n');
            }
        }
        result
    }
}

impl Day10 {
    fn parse(input: &str) -> Vec<Instruction> {
        input
            .trim()
            .split('\n')
            .map(|l| {
                let split: Vec<&str> = l.split(' ').collect();
                if split.len() == 1 {
                    Instruction::Noop
                } else {
                    Instruction::Addx(split[1].parse().unwrap())
                }
            })
            .collect()
    }

    fn run(instructions: &Vec<Instruction>) -> Vec<i32> {
        let mut x = vec![1; 1];
        for ins in instructions {
            match ins {
                Instruction::Noop => x.push(x[x.len() - 1]),
                Instruction::Addx(v) => {
                    x.push(x[x.len() - 1]);
                    x.push(x[x.len() - 1] + v)
                }
            }
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        let soln = Day10 {};
        let input = "noop
addx 3
addx -5
";
        let result = soln.part1(input);
        assert_eq!(result, "0");
    }

    #[test]
    fn part1_ex2() {
        let soln = Day10 {};
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
        let result = soln.part1(input);
        assert_eq!(result, "13140");
    }
}
