use aoc2022::*;
use core::panic;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day;
    let input_dir;
    match args.len() {
        3 => {
            input_dir = args[1].clone();
            day = args[2].parse::<usize>().unwrap()
        }
        _ => panic!(),
    };

    let input = read_input(&input_dir, day);

    let solutions = init();
    let solution = &solutions[day - 1];
    
    let p1 = solution.part1(&input);
    let p2 = solution.part2(&input);

    println!("Part 1:\n{p1}\n\nPart 2:\n{p2}")
}

fn init() -> Vec<Box<dyn Solution>> {
    vec![Box::new(Day1 {})]
}
