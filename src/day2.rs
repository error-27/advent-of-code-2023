use regex::Regex;

pub fn solve(input: &String) -> isize {
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;

    let mut possible: isize = 0;

    let lines = input.lines();

    for (pos, line) in lines.enumerate() {
        let mut impossible = false;
        let red_re = Regex::new(r"[0-9]+ red").unwrap();
        let blue_re = Regex::new(r"[0-9]+ blue").unwrap();
        let green_re = Regex::new(r"[0-9]+ green").unwrap();
        let numbers_re = Regex::new(r"[0-9]+").unwrap();

        let red_matches: Vec<isize> = red_re.find_iter(line)
            .map(|s| numbers_re
            .find(s.as_str())
            .unwrap()
            .as_str().parse::<isize>().unwrap()
        ).collect();

        let blue_matches: Vec<isize> = blue_re.find_iter(line)
            .map(|s| numbers_re
            .find(s.as_str())
            .unwrap()
            .as_str().parse::<isize>().unwrap()
        ).collect();

        let green_matches: Vec<isize> = green_re.find_iter(line)
            .map(|s| numbers_re
            .find(s.as_str())
            .unwrap()
            .as_str().parse::<isize>().unwrap()
        ).collect();

        for r in red_matches {
            if r > red_max {
                impossible = true; 
                break;
            }
        }

        for b in blue_matches {
            if b > blue_max || impossible {
                impossible = true; 
                break;
            }
        }

        for g in green_matches {
            if g > green_max || impossible {
                impossible = true; 
                break;
            }
        }

        if !impossible {
            possible += pos as isize + 1;
        }
    }

    possible
}

pub fn solve2(input: &String) -> isize {
    let mut power: isize = 0;

    let lines = input.lines();

    for (pos, line) in lines.enumerate() {
        let red_re = Regex::new(r"[0-9]+ red").unwrap();
        let blue_re = Regex::new(r"[0-9]+ blue").unwrap();
        let green_re = Regex::new(r"[0-9]+ green").unwrap();
        let numbers_re = Regex::new(r"[0-9]+").unwrap();

        let red_matches: Vec<isize> = red_re.find_iter(line)
            .map(|s| numbers_re
            .find(s.as_str())
            .unwrap()
            .as_str().parse::<isize>().unwrap()
        ).collect();

        let blue_matches: Vec<isize> = blue_re.find_iter(line)
            .map(|s| numbers_re
            .find(s.as_str())
            .unwrap()
            .as_str().parse::<isize>().unwrap()
        ).collect();

        let green_matches: Vec<isize> = green_re.find_iter(line)
            .map(|s| numbers_re
            .find(s.as_str())
            .unwrap()
            .as_str().parse::<isize>().unwrap()
        ).collect();

        let mut highest_red = red_matches.first().unwrap().clone();
        let mut highest_blue = blue_matches.first().unwrap().clone();
        let mut highest_green = green_matches.first().unwrap().clone();

        for r in red_matches {
            highest_red = r.max(highest_red);
        }

        for b in blue_matches {
            highest_blue = b.max(highest_blue);
        }

        for g in green_matches {
            highest_green = g.max(highest_green);
        }

        power += highest_red * highest_blue * highest_green;
    }

    power
}