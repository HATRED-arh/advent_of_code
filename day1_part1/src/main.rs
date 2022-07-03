use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let file = File::open("input.txt").unwrap();
    let data: Vec<i64> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();
    let amount = data
        .iter()
        .zip(data.iter().skip(1))
        .filter(|(current, next)| current < next)
        .count();
    println!("{}", amount)
}
