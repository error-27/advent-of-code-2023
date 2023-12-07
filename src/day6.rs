use regex::Regex;

pub fn solve(input: &String) -> usize {
    let mut solution = 0;

    let re_nums = Regex::new(r"\d+").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let times: Vec<usize> = re_nums.find_iter(lines[0]).map(|m| m.as_str().parse::<usize>().unwrap()).collect();
    let records: Vec<usize> = re_nums.find_iter(lines[1]).map(|m| m.as_str().parse::<usize>().unwrap()).collect();

    let mut wins: Vec<usize> = vec![0; times.len()];
    for r in 0..times.len(){
        println!("calculating race {}...", r);
        println!("current record: {}", records[r]);
        println!("race time: {}", times[r]);
        for i in 1..(times[r]+1)/2 {
            if i*(times[r] - i) > records[r] {
                wins[r] += 1;
            }
        }

        wins[r] = wins[r] * 2;
        if times[r] % 2 == 0 {
            wins[r] += 1;
        }
    }

    solution = wins.iter().product();


    solution
}

pub fn solve2(input: &String) -> usize {
    let mut solution = 0;

    let re_nums = Regex::new(r"\d+").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    // literally just had to remove spaces lmfao
    let times: Vec<usize> = re_nums.find_iter(lines[0].replace(" ", "").as_str()).map(|m| m.as_str().parse::<usize>().unwrap()).collect();
    let records: Vec<usize> = re_nums.find_iter(lines[1].replace(" ", "").as_str()).map(|m| m.as_str().parse::<usize>().unwrap()).collect();

    let mut wins: Vec<usize> = vec![0; times.len()];
    for r in 0..times.len(){
        println!("calculating race {}...", r);
        println!("current record: {}", records[r]);
        println!("race time: {}", times[r]);
        for i in 1..(times[r]+1)/2 {
            if i*(times[r] - i) > records[r] {
                wins[r] += 1;
            }
        }

        wins[r] = wins[r] * 2;
        if times[r] % 2 == 0 {
            wins[r] += 1;
        }
    }

    solution = wins.iter().product();


    solution
}