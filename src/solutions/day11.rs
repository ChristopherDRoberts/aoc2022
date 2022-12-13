use crate::Solution;

pub struct Day11;

impl Solution for Day11 {
    fn part1(&self, input: &str) -> String {
        let (mut items, ops, tests, div, rem) = Day11::init();
        let mut inspects = vec![0; items.len()];
        for round in 0..20 {
            for i in 0..items.len() {
                for j in 0..items[i].len() {
                    inspects[i] += 1;
                    let new = ops[i](items[i][j]) / 3;
                    if new % tests[i] == 0 {
                        items[div[i]].push(new)
                    } else {
                        items[rem[i]].push(new)
                    }
                }
                items[i].clear();
            }
        }
        inspects.sort();
        let result = inspects[inspects.len() - 1] * inspects[inspects.len() - 2];
        format!("{result}")
    }

    fn part2(&self, input: &str) -> String {
        let (mut items, ops, tests, div, rem) = Day11::init();
        let lcm: u64 = tests.iter().product();
        let mut inspects = vec![0u64; items.len()];
        for round in 0..10_000 {
            for i in 0..items.len() {
                for j in 0..items[i].len() {
                    inspects[i] += 1;
                    let new = ops[i](items[i][j]) % lcm;
                    if new % tests[i] == 0 {
                        items[div[i]].push(new)
                    } else {
                        items[rem[i]].push(new)
                    }
                }
                items[i].clear();
            }
        }
        inspects.sort();
        let result = inspects[inspects.len() - 1] * inspects[inspects.len() - 2];
        format!("{result}")
    }
}

impl Day11 {
    fn test_init() -> (
        Vec<Vec<u64>>,
        Vec<Box<dyn Fn(u64) -> u64>>,
        Vec<u64>,
        Vec<usize>,
        Vec<usize>,
    ) {
        let items = vec![
            vec![79, 98],
            vec![54, 65, 75, 74],
            vec![79, 60, 97],
            vec![74],
        ];
        let ops: Vec<Box<dyn Fn(u64) -> u64>> = vec![
            Box::new(|x| x * 19),
            Box::new(|x| x + 6),
            Box::new(|x| x * x),
            Box::new(|x| x + 3),
        ];
        let tests = vec![23, 19, 13, 17];
        let div = vec![2, 2, 1, 0];
        let rem = vec![3, 0, 3, 1];
        (items, ops, tests, div, rem)
    }

    fn init() -> (
        Vec<Vec<u64>>,
        Vec<Box<dyn Fn(u64) -> u64>>,
        Vec<u64>,
        Vec<usize>,
        Vec<usize>,
    ) {
        let items = vec![
            vec![66, 71, 94],
            vec![70],
            vec![62, 68, 56, 65, 94, 78],
            vec![89, 94, 94, 67],
            vec![71, 61, 73, 65, 98, 98, 63],
            vec![55, 62, 68, 61, 60],
            vec![93, 91, 69, 64, 72, 89, 50, 71],
            vec![76, 50],
        ];
        let ops: Vec<Box<dyn Fn(u64) -> u64>> = vec![
            Box::new(|x| x * 5),
            Box::new(|x| x + 6),
            Box::new(|x| x + 5),
            Box::new(|x| x + 2),
            Box::new(|x| x * 7),
            Box::new(|x| x + 7),
            Box::new(|x| x + 1),
            Box::new(|x| x * x),
        ];
        let tests = vec![3, 17, 2, 19, 11, 5, 13, 7];
        let div = vec![7, 3, 3, 7, 5, 2, 5, 4];
        let rem = vec![4, 0, 1, 0, 6, 1, 2, 6];
        (items, ops, tests, div, rem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn part1_ex1() {
        let soln = Day11 {};
        let input = "";
        let result = soln.part1(input);
        assert_eq!(result, "10605");
    }

    #[test]
    #[ignore]
    fn part2_ex1() {
        let soln = Day11 {};
        let input = "";
        let result = soln.part2(input);
        assert_eq!(result, "2713310158");
    }
}
