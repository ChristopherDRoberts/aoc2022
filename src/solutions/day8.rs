use crate::Solution;

pub struct Day8;

impl Solution for Day8 {
    fn part1(&self, input: &str) -> String {
        let grid = Day8::parse_input(input);
        let tgrid = Day8::tranpose(&grid);
        let mut result = 2 * grid.len() + 2 * (grid.len() - 2);
        for row in 1..grid.len() - 1 {
            for col in 1..grid[0].len() - 1 {
                if Day8::visible(row, col, &grid, &tgrid) {
                    result += 1
                }
            }
        }
        format!("{result}")
    }

    fn part2(&self, input: &str) -> String {
        let grid = Day8::parse_input(input);
        let mut result = 0;
        for row in 1..grid.len() - 1 {
            for col in 1..grid[0].len() - 1 {
                let score = Day8::score(row, col, &grid);
                if score >= result {
                    result = score
                };
            }
        }
        format!("{result}")
    }
}

impl Day8 {
    fn parse_input(input: &str) -> Vec<Vec<u32>> {
        input
            .trim()
            .split('\n')
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect()
    }

    fn visible(row: usize, col: usize, grid: &Vec<Vec<u32>>, tgrid: &Vec<Vec<u32>>) -> bool {
        let tree = &grid[row][col];
        if (grid[row][0..col]).iter().max().unwrap() < tree {
            return true;
        }
        if (grid[row][col + 1..]).iter().max().unwrap() < tree {
            return true;
        }
        if tgrid[col][0..row].iter().max().unwrap() < tree {
            return true;
        }
        if tgrid[col][row + 1..].iter().max().unwrap() < tree {
            return true;
        }
        false
    }

    fn score(row: usize, col: usize, grid: &Vec<Vec<u32>>) -> u32 {
        let tree = grid[row][col];
        let mut up = 0;
        let mut down = 0;
        let mut left = 0;
        let mut right = 0;
        let m = grid.len();
        let n = grid[0].len();

        for j in (0..col).rev() {
            if grid[row][j] < tree {
                left += 1
            }
            if grid[row][j] >= tree {
                left += 1;
                break;
            }
        }

        for j in col + 1..n {
            if grid[row][j] < tree {
                right += 1
            }
            if grid[row][j] >= tree {
                right += 1;
                break;
            }
        }

        for i in (0..row).rev() {
            if grid[i][col] < tree {
                up += 1
            }
            if grid[i][col] >= tree {
                up += 1;
                break;
            }
        }

        for i in row + 1..m {
            if grid[i][col] < tree {
                down += 1
            }
            if grid[i][col] >= tree {
                down += 1;
                break;
            }
        }
        
        up * down * left * right
    }

    fn tranpose(grid: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut tgrid = vec![vec![0; m]; n];
        for i in 0..m {
            for j in 0..n {
                tgrid[j][i] = grid[i][j];
            }
        }
        tgrid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        let soln = Day8 {};
        let input = "30373
25512
65332
33549
35390
";
        let result = soln.part1(input);
        assert_eq!(result, "21");
    }

    #[test]
    fn part2_ex1() {
        let soln = Day8 {};
        let input = "30373
25512
65332
33549
35390
";
        let result = soln.part2(input);
        assert_eq!(result, "8");
    }
}
