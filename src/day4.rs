use regex::Regex;

pub fn solve(input: &String) -> isize {
    let mut solution = 0;

    let re_sets = Regex::new(r"( +\d+)+").unwrap();
    let re_nums = Regex::new(r"\d+").unwrap();

    let lines = input.lines();

    for line in lines {

        let sets = re_sets.find_iter(line);
        let mut winning: Vec<isize> = Vec::new();
        let mut nums: Vec<isize> = Vec::new();
        for (pos, m) in sets.enumerate() {
            if pos == 0 { continue; } // captures game number too, so discard it from searching

            let n: Vec<isize> = re_nums
                .find_iter(m.as_str())
                .map(|i| i.as_str().parse::<isize>().unwrap())
                .collect();

            if pos == 1 {
                winning = n;
            }else{
                nums = n;
            }
        }

        let mut won: isize = 0;
        //println!("Winning: {:?}", winning);
        //println!("Actual: {:?}", nums);

        for w in winning {
            if nums.contains(&w) {
                won += 1;
            }
        }

        if won == 0 { continue; }

        let score: isize = 1 * 2_isize.pow(won as u32 - 1);
        //println!("Score: {}", score);
    
        solution += score;
    }

    solution
}

pub fn solve2(input: &String) -> isize {
    let mut solution = 0;

    let re_sets = Regex::new(r"( +\d+)+").unwrap();
    let re_nums = Regex::new(r"\d+").unwrap();

    let lines = input.lines();

    let mut totals: Vec<isize> = vec![0; 201];
    let mut originals: isize = 0;

    for (pos, line) in lines.enumerate() {

        originals += 1;

        let sets = re_sets.find_iter(line);
        let mut winning: Vec<isize> = Vec::new();
        let mut nums: Vec<isize> = Vec::new();
        for (pos, m) in sets.enumerate() {
            if pos == 0 { continue; } // captures game number too, so discard it from searching

            let n: Vec<isize> = re_nums
                .find_iter(m.as_str())
                .map(|i| i.as_str().parse::<isize>().unwrap())
                .collect();

            if pos == 1 {
                winning = n;
            }else{
                nums = n;
            }
        }

        let mut won: isize = 0;
        //println!("Winning: {:?}", winning);
        //println!("Actual: {:?}", nums);

        for w in winning {
            if nums.contains(&w) {
                won += 1;
            }
        }
        println!("Won {} for round {}", won, pos);

        if won == 0 { continue; }

        for i in pos+1..pos+won as usize+1 {
            totals[i] += totals[pos] + 1;
        }
        println!("Totals for round {}: {:?}", pos, totals);
    }

    solution = originals + totals.iter().sum::<isize>();

    solution
}