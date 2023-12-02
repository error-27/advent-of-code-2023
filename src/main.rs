use std::io::Result;
mod day1;

fn main() -> Result<()> {
    let content = include_str!("inputs/day1").to_string();

    println!("{}", day1::solve(&content));
    println!("{}", day1::solve2(&content));

    Ok(())
}
