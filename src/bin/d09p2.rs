use advent_2023::day_09::p2;
use std::io;

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock();
    let output = io::stdout();
    p2::solve(input, output);
}