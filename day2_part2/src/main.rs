use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal = 0;
    let _: () = BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .split_once(' ')
                .map(|(key, value)| (key.to_owned(), value.parse::<i64>().unwrap()))
        })
        .map(|result| {
            let (direction, num) = result.unwrap();
            match direction.as_ref() {
                "down" => aim += num,
                "up" => aim -= num,
                "forward" => {
                    horizontal += num;
                    depth += num * aim
                }
                _ => (),
            }
        })
        .collect();

    println!("{}", depth * horizontal);
}
