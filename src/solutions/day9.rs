use crate::Solution;
use std::collections::HashSet;

enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

pub struct Day9;

impl Solution for Day9 {
    fn part1(&self, input: &str) -> String {
        Day9::soln_impl(input, 2)
    }

    fn part2(&self, input: &str) -> String {
        Day9::soln_impl(input, 10)
    }
}

impl Day9 {
    fn soln_impl(input: &str, n: usize) -> String {
        let moves = Day9::parse_input(input);
        let mut points = vec![Point { x: 0, y: 0 }; n];
        let mut visited = HashSet::new();
        visited.insert(points[n - 1]);

        for mov in moves {
            match mov {
                Move::Up(steps) => {
                    for _ in 0..steps {
                        Day9::update(0, 1, &mut points, &mut visited);
                    }
                }

                Move::Down(steps) => {
                    for _ in 0..steps {
                        Day9::update(0, -1, &mut points, &mut visited);
                    }
                }

                Move::Left(steps) => {
                    for _ in 0..steps {
                        Day9::update(-1, 0, &mut points, &mut visited);
                    }
                }

                Move::Right(steps) => {
                    for _ in 0..steps {
                        Day9::update(1, 0, &mut points, &mut visited);
                    }
                }
            }
        }
        let result = visited.len();
        format!("{result}")
    }

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

    fn adjacent(head: &Point, tail: &Point) -> bool {
        (head.x - tail.x).abs() <= 1 && (head.y - tail.y).abs() <= 1
    }

    fn step(head: &Point, tail: &Point) -> Point {
        let dx = (head.x - tail.x).signum();
        let dy = (head.y - tail.y).signum();
        Point {
            x: tail.x + dx,
            y: tail.y + dy,
        }
    }

    fn update(dx: i32, dy: i32, points: &mut Vec<Point>, visited: &mut HashSet<Point>) {
        points[0] = Point {
            x: points[0].x + dx,
            y: points[0].y + dy,
        };
        for i in 1..points.len() {
            if !Day9::adjacent(&points[i - 1], &points[i]) {
                points[i] = Day9::step(&points[i - 1], &points[i]);
                if i == points.len() - 1 {
                    visited.insert(points[i]);
                }
            }
        }
    }
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
