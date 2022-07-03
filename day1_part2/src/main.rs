use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let data: Vec<i64> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();
    let windows = data.windows(3);
    let amount = windows
        .clone()
        .zip(windows.skip(1))
        .filter(|(current, next)| -> bool {
            let sum_1: i64 = current.iter().sum();
            let sum_2: i64 = next.iter().sum();
            sum_1 < sum_2
        })
        .count();

    println!("{}", amount)
}
