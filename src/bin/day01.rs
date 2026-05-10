// Day 1: No Time for a Taxicab

use std::fs;
use std::path::absolute;

fn main() {
    let mut d: char = 'N';
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let input = fs::read_to_string("inputs/day01.txt").expect("Failed to read the input file");

    for instruction in input.split(", ") {
        let direction: char = instruction.chars().next().unwrap();
        let distance: i32 = instruction[1..]
            .parse()
            .expect("Failed to parse distance into an integer");
        println!("{} {} {}", instruction, direction, distance);

        if direction == 'R' {
            d = match d {
                'N' => 'E',
                'E' => 'S',
                'S' => 'W',
                'W' => 'N',
                _ => d, // Wildcard safety fallback required by Rust
            };
        } else if direction == 'L' {
            d = match d {
                'N' => 'W',
                'W' => 'S',
                'S' => 'E',
                'E' => 'N',
                _ => d, // Wildcard safety fallback required by Rust
            };
        }

        match d {
            'E' => x += distance,
            'S' => y += distance,
            'W' => x -= distance,
            'N' => y -= distance,
            _ => {}
        }
    }
    println!("x: {}, y: {}", x, y);
    let final_dist = x.abs() + y.abs();
    println!("final_dist: {}", final_dist)
}
