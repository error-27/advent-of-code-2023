use regex::{Regex, Match};

const WIDTH: usize = 139;
const HEIGHT: usize = 139;

pub fn solve(input: &String) -> isize {
    let mut solution: isize = 0;

    let re = Regex::new(r"\d+").unwrap();
    let mut numbers: Vec<Vec<Match>> = Vec::new();
    let lines = input.lines();

    for (y, line) in lines.enumerate() {
        let results = re.find_iter(line);

        for m in results {
            let mut counted = false;
            let x = m.start();
            let x_end = m.end();

            if x > 0 && line.chars().nth(x - 1).unwrap() != '.' { // left
                counted = true;
            }
            if !counted && x_end - 1 < WIDTH && line.chars().nth(x_end).unwrap() != '.' { // right
                counted = true;
            }

            if !counted && y > 0 { // symbols above
                let above = input.lines().nth(y-1).unwrap();
                for ix in x..x_end {
                    if above.chars().nth(ix).unwrap() != '.' {
                        counted = true;
                        break;
                    }
                }
            }

            if !counted && y < HEIGHT { // symbols below
                let below = input.lines().nth(y+1).unwrap();
                for ix in x..x_end {
                    if below.chars().nth(ix).unwrap() != '.' {
                        counted = true;
                        break;
                    }
                }
            }

            if !counted && y > 0 && x > 0 {
                let above = input.lines().nth(y-1).unwrap();
                println!("checking {}, {} for {}", x-1, y-1, m.as_str());
                if above.chars().nth(x-1).unwrap() != '.' {
                    counted = true;
                }
            }
            if !counted && y > 0 && x_end - 1 < WIDTH {
                let above = input.lines().nth(y-1).unwrap();
                println!("checking {}, {} for {}", x_end, y-1, m.as_str());
                if above.chars().nth(x_end).unwrap() != '.' {
                    counted = true;
                }
            }
            if !counted && y < HEIGHT && x > 0 {
                let below = input.lines().nth(y+1).unwrap();
                println!("checking {}, {} for {}", x-1, y+1, m.as_str());
                if below.chars().nth(x-1).unwrap() != '.' {
                    counted = true;
                }
            }
            if !counted && y < HEIGHT && x_end - 1 < WIDTH {
                let below = input.lines().nth(y+1).unwrap();
                println!("checking {}, {} for {}", x_end, y+1, m.as_str());
                if below.chars().nth(x_end).unwrap() != '.' {
                    counted = true;
                }
            }

            if counted {
                println!("verified {}", m.as_str());
                solution += m.as_str().parse::<isize>().unwrap();
            }
        }
    }

    solution
}

pub fn solve2(input: &String) -> isize {
    let mut solution: isize = 0;

    solution
}