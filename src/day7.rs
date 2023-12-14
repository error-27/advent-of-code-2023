pub fn solve(input: &String) -> isize {
    let mut solution = 0;
    let cards = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
    
    let mut hands: Vec<String> = vec![];
    let mut bids: Vec<isize> = vec![];

    let lines = input.lines();

    for line in lines {
        let s: Vec<&str> = line.split(" ").collect();
        hands.push(s[0].to_string());
        bids.push(s[1].parse::<isize>().unwrap());
    }

    let mut ranks: Vec<isize> = vec![];

    for pass in 0..hands.len()-1 {
        
    }

    solution
}

pub fn solve2(input: &String) -> isize {
    let mut solution = 0;

    solution
}

fn weaker(a: &String, b: &String) -> bool {
    let cards = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
    let mut most_c_a: char = a.chars().nth(0).unwrap();
    let mut c_num_a: usize = 0;
    for c in cards {
        if !a.contains(c) {continue;}
        let found = a.matches(c).count();
        if found > c_num_a {
            c_num_a = found;
            most_c_a = c;
        }
    }

    let mut most_c_b: char = b.chars().nth(0).unwrap();
    let mut c_num_b: usize = 0;
    for c in cards {
        if !b.contains(c) {continue;}
        let found = b.matches(c).count();
        if found > c_num_b {
            c_num_b = found;
            most_c_b = c;
        }
    }

    if most_c_a < most_c_b {
        return true;
    }else if most_c_b < most_c_a {
        return false;
    }

    false
}