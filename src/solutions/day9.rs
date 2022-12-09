use std::collections::HashSet;

use crate::Solution;

pub struct Day9;

impl Solution for Day9 {
    fn part1(&self, input: &str) -> String {
        let moves = Day9::parse_input(input);
        let mut head = (0, 0);
        let mut tail = (0, 0);
        let mut visited = HashSet::new();
        visited.insert(tail);

        for mov in moves {
            match mov {
                Move::Up(steps) => {
                    for _ in 0..steps {
                        let prev = head;
                        head = (head.0, head.1 + 1);
                        if !Day9::adjacent(head, tail) {
                            tail = prev;
                            visited.insert(tail);
                        }
                    }
                }

                Move::Down(steps) => {
                    for _ in 0..steps {
                        let prev = head;
                        head = (head.0, head.1 - 1);
                        if !Day9::adjacent(head, tail) {
                            tail = prev;
                            visited.insert(tail);
                        }
                    }
                }

                Move::Left(steps) => {
                    for _ in 0..steps {
                        let prev = head;
                        head = (head.0 - 1, head.1);
                        if !Day9::adjacent(head, tail) {
                            tail = prev;
                            visited.insert(tail);
                        }
                    }
                }

                Move::Right(steps) => {
                    for _ in 0..steps {
                        let prev = head;
                        head = (head.0 + 1, head.1);
                        if !Day9::adjacent(head, tail) {
                            tail = prev;
                            visited.insert(tail);
                        }
                    }
                }
            }
        }
        let result = visited.len();
        format!("{result}")
    }

    fn part2(&self, input: &str) -> String {
        let moves = Day9::parse_input(input);
        let mut points = vec![(0, 0); 10];
        let mut visited = HashSet::new();
        visited.insert(points[9]);

        for mov in moves {
            match mov {
                Move::Up(steps) => {
                    for _ in 0..steps {
                        points[0] = (points[0].0, points[0].1 + 1);
                        for i in 1..points.len() {
                            if !Day9::adjacent(points[i - 1], points[i]) {
                                points[i] = Day9::step(points[i - 1], points[i]);
                                if i == points.len() - 1 {
                                    visited.insert(points[i]);
                                }
                            }
                        }
                    }
                }

                Move::Down(steps) => {
                    for _ in 0..steps {
                        points[0] = (points[0].0, points[0].1 - 1);
                        for i in 1..points.len() {
                            if !Day9::adjacent(points[i - 1], points[i]) {
                                points[i] = Day9::step(points[i - 1], points[i]);
                                if i == points.len() - 1 {
                                    visited.insert(points[i]);
                                }
                            }
                        }
                    }
                }

                Move::Left(steps) => {
                    for _ in 0..steps {
                        points[0] = (points[0].0 - 1, points[0].1);
                        for i in 1..points.len() {
                            if !Day9::adjacent(points[i - 1], points[i]) {
                                points[i] = Day9::step(points[i - 1], points[i]);
                                if i == points.len() - 1 {
                                    visited.insert(points[i]);
                                }
                            }
                        }
                    }
                }

                Move::Right(steps) => {
                    for _ in 0..steps {
                        points[0] = (points[0].0 + 1, points[0].1);
                        for i in 1..points.len() {
                            if !Day9::adjacent(points[i - 1], points[i]) {
                                points[i] = Day9::step(points[i - 1], points[i]);
                                if i == points.len() - 1 {
                                    visited.insert(points[i]);
                                }
                            }
                        }
                    }
                }
            }
        }
        let result = visited.len();
        format!("{result}")
    }
}

impl Day9 {
    fn parse_input(input: &str) -> Vec<Move> {
        input
            .trim()
            .split('\n')
            .map(|l| {
                let mut split = l.split(' ');
                let dir = split.next().unwrap();
                let steps = split.next().unwrap().parse().unwrap();
                match dir {
                    "U" => Move::Up(steps),
                    "D" => Move::Down(steps),
                    "L" => Move::Left(steps),
                    "R" => Move::Right(steps),
                    _ => panic!(),
                }
            })
            .collect()
    }

    fn adjacent(head: (i32, i32), tail: (i32, i32)) -> bool {
        (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1
    }

    fn step(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
        let dx = (head.0 - tail.0).signum();
        let dy = (head.1 - tail.1).signum();
        (tail.0 + dx, tail.1 + dy)
    }
}

enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        let soln = Day9 {};
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
        let result = soln.part1(input);
        assert_eq!(result, "13")
    }

    #[test]
    fn part2_ex1() {
        let soln = Day9 {};
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
        let result = soln.part2(input);
        assert_eq!(result, "1")
    }

    #[test]
    fn part2_ex2() {
        let soln = Day9 {};
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20        
";
        let result = soln.part2(input);
        assert_eq!(result, "36")
    }
}
