use crate::Solution;
use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    idx: usize,
    dist: u32,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.dist < other.dist {
            return Ordering::Greater;
        }
        if self.dist > other.dist {
            return Ordering::Less;
        }
        if self.idx < other.idx {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }
}

pub struct Day12;

impl Solution for Day12 {
    fn part1(&self, input: &str) -> String {
        let (grid, start, end) = Day12::parse(input);
        let graph = Day12::build_graph(grid);
        let result = Day12::shortest_path(&graph, start, end);
        format!("{result}")
    }

    fn part2(&self, input: &str) -> String {
        let (grid, start, end) = Day12::parse(input);
        let graph = Day12::build_graph(grid);
        let candidates = Day12::other_starts(input);
        let mut result = Day12::shortest_path(&graph, start, end);
        for candidate in candidates {
            let dist = Day12::shortest_path(&graph, candidate, end);
            if dist < result {
                result = dist
            };
        }

        format!("{result}")
    }
}

impl Day12 {
    fn parse(input: &str) -> (Vec<Vec<char>>, usize, usize) {
        let start = input
            .replace("\n", "")
            .char_indices()
            .find(|(_, c)| *c == 'S')
            .unwrap()
            .0;
        let end = input
            .replace("\n", "")
            .char_indices()
            .find(|(_, c)| *c == 'E')
            .unwrap()
            .0;
        let grid = input
            .trim()
            .split('\n')
            .map(|l| l.replace("S", "a").replace("E", "z").chars().collect())
            .collect();
        (grid, start, end)
    }

    fn other_starts(input: &str) -> Vec<usize> {
        input
            .replace("\n", "")
            .char_indices()
            .filter(|(_, c)| *c == 'a')
            .map(|(i, _)| i)
            .collect()
    }

    fn shortest_path(graph: &Vec<Vec<usize>>, start: usize, end: usize) -> u32 {
        let n = graph.len();
        let mut dists = vec![u32::MAX; n];
        dists[start] = 0;
        let mut queue = BinaryHeap::new();
        for i in 0..graph.len() {
            queue.push(State {
                idx: i,
                dist: dists[i],
            });
        }

        while let Some(u) = queue.pop() {
            let i = u.idx;
            for v in &graph[i] {
                let test = dists[i] + 1;
                if test < dists[*v] {
                    dists[*v] = test;
                    queue.push(State {
                        idx: *v,
                        dist: test,
                    })
                }
            }
        }

        dists[end]
    }

    fn build_graph(grid: Vec<Vec<char>>) -> Vec<Vec<usize>> {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut graph = vec![Vec::new(); grid.len() * grid[0].len()];
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                let height = grid[row][col] as u32;
                let idx = Day12::point_to_idx((row, col), cols);
                // left
                if col >= 1 {
                    if grid[row][col - 1] as u32 <= height + 1 {
                        graph[idx].push(Day12::point_to_idx((row, col - 1), cols))
                    }
                }
                // right
                if col < cols - 1 {
                    if grid[row][col + 1] as u32 <= height + 1 {
                        graph[idx].push(Day12::point_to_idx((row, col + 1), cols))
                    }
                }
                // up
                if row >= 1 {
                    if grid[row - 1][col] as u32 <= height + 1 {
                        graph[idx].push(Day12::point_to_idx((row - 1, col), cols))
                    }
                }
                // down
                if row < rows - 1 {
                    if grid[row + 1][col] as u32 <= height + 1 {
                        graph[idx].push(Day12::point_to_idx((row + 1, col), cols))
                    }
                }
            }
        }
        graph
    }

    fn point_to_idx(point: (usize, usize), cols: usize) -> usize {
        let (row, col) = point;
        row * cols + col
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        let soln = Day12 {};
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
        let result = soln.part1(input);
        assert_eq!(result, "31");
    }

    #[test]
    fn part2_ex1() {
        let soln = Day12 {};
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
        let result = soln.part2(input);
        assert_eq!(result, "29");
    }
}
