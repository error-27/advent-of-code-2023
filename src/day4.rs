use regex::Regex;

pub fn solve(input: &String) -> usize {
    let mut solution = 0;

    let re_sets = Regex::new(r"( +\d+)+").unwrap();
    let re_nums = Regex::new(r"\d+").unwrap();

    let lines = input.lines();

    for line in lines {

        let sets = re_sets.find_iter(line);
        let mut winning: Vec<usize> = Vec::new();
        let mut nums: Vec<usize> = Vec::new();
        for (pos, m) in sets.enumerate() {
            if pos == 0 { continue; } // captures game number too, so discard it from searching

            let n: Vec<usize> = re_nums
                .find_iter(m.as_str())
                .map(|i| i.as_str().parse::<usize>().unwrap())
                .collect();

            if pos == 1 {
                winning = n;
            }else{
                nums = n;
            }
        }

        let mut won: usize = 0;
        println!("Winning: {:?}", winning);
        println!("Actual: {:?}", nums);

        for w in winning {
            if nums.contains(&w) {
                won += 1;
            }
        }

        if won == 0 { continue; }

        let score: usize = 1 * 2_usize.pow(won as u32 - 1);
        println!("Score: {}", score);
    
        solution += score;
    }

    solution
}