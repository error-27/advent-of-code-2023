use std::io::{Result, ErrorKind, BufReader, prelude::*};
use std::env;
use std::fs::File;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

mod day9;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("please choose a day and part");
        return Err(ErrorKind::Other.into());
    }

    let day = args[1].parse::<u8>().unwrap_or(1);
    let part2 = args[2].parse::<bool>().unwrap_or(false);

    let mut file = File::open(format!("./inputs/day{}", day))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let solution = match day {
        1 => if !part2 {day1::solve(&contents)} else {day1::solve2(&contents)},
        2 => if !part2 {day2::solve(&contents)} else {day2::solve2(&contents)},
        3 => if !part2 {day3::solve(&contents)} else {day3::solve2(&contents)},
        4 => if !part2 {day4::solve(&contents)} else {day4::solve2(&contents)},
        5 => if !part2 {day5::solve(&contents)} else {day5::solve2(&contents)},
        6 => if !part2 {day6::solve(&contents)} else {day6::solve2(&contents)},
        7 => if !part2 {day7::solve(&contents)} else {day7::solve2(&contents)},
        9 => if !part2 {day9::solve(&contents)} else {day9::solve2(&contents)},
        _ => 0
    };

    println!("Day {}", day);
    if part2 { println!("Part 2") } else { println!("Part 1") }
    println!("Solution: {}", solution);

    Ok(())
}
