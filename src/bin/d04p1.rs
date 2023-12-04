use advent_2023::day_04::p1;
use std::io;

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock();
    let output = io::stdout();
    p1::solve(input, output);
}