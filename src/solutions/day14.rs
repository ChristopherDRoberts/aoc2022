use std::collections::HashSet;

use crate::Solution;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

struct Path {
    nodes: Vec<Point>,
}

impl Path {
    fn bottom(&self) -> i32 {
        self.nodes.iter().map(|p| p.y).max().unwrap()
    }

    fn points(&self) -> HashSet<Point> {
        let mut points = HashSet::new();
        for i in 0..self.nodes.len() - 1 {
            let start = self.nodes[i];
            let end = self.nodes[i + 1];
            points.insert(start);
            points.insert(end);
            if start.y == end.y {
                for new_x in start.x.min(end.x)..start.x.max(end.x) {
                    points.insert(Point {
                        x: new_x,
                        y: start.y,
                    });
                }
            }

            if start.x == end.x {
                for new_y in start.y.min(end.y)..start.y.max(end.y) {
                    points.insert(Point {
                        x: start.x,
                        y: new_y,
                    });
                }
            }
        }
        return points;
    }
}

struct Scan {
    paths: Vec<Path>,
}

impl Scan {
    fn bottom(&self) -> i32 {
        self.paths.iter().map(|p| p.bottom()).max().unwrap()
    }

    fn points(&self) -> HashSet<Point> {
        let mut points = HashSet::new();
        for path in &self.paths {
            points.extend(path.points())
        }
        points
    }
}
pub struct Day14;

impl Solution for Day14 {
    fn part1(&self, input: &str) -> String {
        let scan = Day14::parse(input);
        let bottom = scan.bottom();
        let mut points = scan.points();
        let mut dropped = 0;
        while Day14::drop_grain(&mut points, bottom) {
            dropped += 1;
        }
        format!("{dropped}")
    }

    fn part2(&self, input: &str) -> String {
        let source = Point{x: 500, y: 0};
        let scan = Day14::parse(input);
        let bottom = scan.bottom();
        let mut points = scan.points();
        Day14::add_floor(&mut points, bottom, 10_000);
        let mut dropped = 0;
        while !points.contains(&source) {
            Day14::drop_grain(&mut points, bottom);
            dropped += 1;
        }
        format!("{dropped}")
    }
}

impl Day14 {
    fn parse(input: &str) -> Scan {
        let mut paths = Vec::new();
        for line in input.trim().split('\n') {
            let points = line
                .split(" -> ")
                .map(|p| {
                    let parts: Vec<&str> = p.split(',').collect();
                    Point {
                        x: parts[0].parse().unwrap(),
                        y: parts[1].parse().unwrap(),
                    }
                })
                .collect();
            let path = Path { nodes: points };
            paths.push(path);
        }
        Scan { paths }
    }

    fn drop_grain(points: &mut HashSet<Point>, bottom: i32) -> bool {
        let mut grain = Point { x: 500, y: 0 };
        while grain.y < bottom + 3 {
            // down
            grain.y += 1;
            if !points.contains(&grain) {
                continue;
            }
            // down-left
            grain.x -= 1;
            if !points.contains(&grain) {
                continue;
            }
            // down-right
            grain.x += 2;
            if !points.contains(&grain) {
                continue;
            }
            // settle
            grain.y -= 1;
            grain.x -= 1;
            points.insert(grain);
            return true;
        }
        return false;
    }

    fn add_floor(points: &mut HashSet<Point>, bottom: i32, width: u32) {
        let x = 500i32;
        let y = bottom + 2;
        for i in 0..width / 2 {
            points.insert(Point {
                x: x + (i as i32),
                y,
            });
            points.insert(Point {
                x: x - (i as i32),
                y,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        let soln = Day14 {};
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";
        let result = soln.part1(input);
        assert_eq!(result, "24")
    }

    #[test]
    fn part2_ex1() {
        let soln = Day14 {};
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";
        let result = soln.part2(input);
        assert_eq!(result, "93")
    }
}
