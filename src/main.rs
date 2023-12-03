use std::io::Result;
mod day1;
mod day2;
mod day3;

fn main() -> Result<()> {
    let d1_file = include_str!("inputs/day1").to_string();
    let d2_file = include_str!("inputs/day2").to_string();
    let d3_file = include_str!("inputs/day3").to_string();

    //println!("Day 1-1: {}", day1::solve(&d1_file));
    //println!("Day 1-2: {}", day1::solve2(&d1_file));

    //println!("Day 2-1: {}", day2::solve(&d2_file));
    //println!("Day 2-2: {}", day2::solve2(&d2_file));

    println!("Day 3-1: {}", day3::solve(&d3_file));

    Ok(())
}
