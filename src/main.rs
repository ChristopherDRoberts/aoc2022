use aoc2022::*;
use colored::Colorize;
use core::panic;
use rand::prelude::*;
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

    println!();
    print_tree(day);
    println!();
    println!("Part 1:\n{p1}\n\nPart 2:\n{p2}");
}

fn init() -> Vec<Box<dyn Solution>> {
    vec![
        Box::new(Day1 {}),
        Box::new(Day2 {}),
        Box::new(Day3 {}),
        Box::new(Day4 {}),
    ]
}

fn print_tree(height: usize) {
    let leaf = "#".green();
    let star = "X".truecolor(255, 255, 0);
    let bark = "#".truecolor(175, 50, 0);
    let red = "@".truecolor(255, 0, 0);
    let blue = "$".blue();
    let blank = " ".black();
    let newline = "\n".black();

    let red_prob = 0.1;
    let blue_prob = 0.1;

    let mut tree = Vec::new();

    // star
    let mut row = Vec::new();
    for _ in 0..(height - 1) {
        row.push(&blank);
    }
    row.push(&star);
    for _ in 0..(height - 1) {
        row.push(&blank);
    }
    row.push(&newline);
    tree.push(row);

    let mut rng = rand::thread_rng();
    for i in 0..height {
        let mut row = Vec::new();
        for _ in 0..(height - i - 1) {
            row.push(&blank);
        }
        for _ in 0..(2 * i + 1) {
            let p: f64 = rng.gen();
            if p <= red_prob {
                row.push(&red)
            } else if p > red_prob && p <= red_prob + blue_prob {
                row.push(&blue)
            } else {
                row.push(&leaf)
            }
        }
        for _ in 0..(height - i - 1) {
            row.push(&blank);
        }
        row.push(&newline);
        tree.push(row)
    }

    //bark
    let mut row = Vec::new();
    for _ in 0..(height - 1) {
        row.push(&blank);
    }
    row.push(&bark);
    for _ in 0..(height - 1) {
        row.push(&blank);
    }
    row.push(&newline);
    tree.push(row);

    for row in tree {
        for col in row {
            print!("{}", col);
        }
    }
}
