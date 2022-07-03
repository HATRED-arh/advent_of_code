use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut hash = HashMap::new();
    let _: () = BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .split_once(' ')
                .map(|(key, value)| (key.to_owned(), value.parse::<i64>().unwrap()))
        })
        .map(|result| {
            let (direction, num) = result.unwrap();
            *hash.entry(direction).or_insert(0) += num;
        })
        .collect();
    println!(
        "{}",
        (hash.get("down").unwrap() - hash.get("up").unwrap()) * hash.get("forward").unwrap()
    )
}
