use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let (mut gamma, mut epsilon) = (String::new(), String::new());
    let data = BufReader::new(file)
        .lines()
        .map(|binary| binary.unwrap())
        .collect::<Vec<_>>();

    for i in 0..data.get(0).unwrap().len() {
        let entries: String = data
            .iter()
            .map(|item| item.chars().nth(i).unwrap())
            .collect();
        let condition = entries.matches('0').count() > data.len() / 2;
        gamma.push(if condition { '0' } else { '1' });
        epsilon.push(if condition { '1' } else { '0' });
    }
    println!(
        "{}",
        isize::from_str_radix(&gamma, 2)? * isize::from_str_radix(&epsilon, 2)?
    );
    Ok(())
}
