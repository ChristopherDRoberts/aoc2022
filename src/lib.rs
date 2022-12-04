use std::fs;

pub mod solutions;

pub trait Solution {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

pub fn read_input(input_dir: &str, day: usize) -> String {
    let file_path = format!("{input_dir}/day{day}.txt");
    fs::read_to_string(file_path).unwrap()
}