use crate::Solution;
use regex::Regex;
use std::collections::HashMap;

pub struct Day7;

impl Solution for Day7 {
    fn part1(&self, input: &str) -> String {
        let sizes = Day7::dir_sizes(input);
        let mut result = 0;
        for v in sizes.values() {
            if *v <= 100_000 {
                result += v
            };
        }
        format!("{result}")
    }

    fn part2(&self, input: &str) -> String {
        let sizes = Day7::dir_sizes(input);
        let space = 30_000_000 - (70_000_000 - sizes.values().max().unwrap());
        let result = sizes.values().filter(|x| **x >= space).min().unwrap();
        format!("{result}")
    }
}

impl Day7 {
    fn dir_sizes(input: &str) -> HashMap<String, u32> {
        let cd_re = Regex::new(r"\$ cd (/|[a-z]+)").unwrap();
        let file_re = Regex::new(r"(\d+).*").unwrap();
        let mut dirs = Vec::new();
        let mut sizes = HashMap::new();
        for line in input.trim().split('\n') {
            if let Some(caps) = cd_re.captures(line) {
                let cd = &caps[1];
                let mut full_path = dirs.join("/");
                full_path.push_str(cd);
                dirs.push(full_path.clone());
                if !sizes.contains_key(&full_path) {
                    sizes.insert(full_path.clone(), 0);
                }
                continue;
            }
            if line == "$ cd .." {
                dirs.pop();
                continue;
            }
            if let Some(caps) = file_re.captures(line) {
                let size: u32 = caps[1].parse().unwrap();
                for k in &dirs {
                    *(sizes.get_mut(k).unwrap()) += size;
                }
            }
        }
        sizes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        let soln = Day7 {};
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";
        let result = soln.part1(input);
        assert_eq!(result, "95437");
    }

    #[test]
    fn part2_ex1() {
        let soln = Day7 {};
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";
        let result = soln.part2(input);
        assert_eq!(result, "24933642");
    }
}
