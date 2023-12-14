use regex::Regex;

pub fn solve(input: &String) -> isize {
    let mut solution = 0;

    let lines = input.lines();
    let nums_re = Regex::new(r"-?\d+").unwrap();

    for line in lines {
        let nums: Vec<isize> = nums_re.find_iter(line).map(|m| m.as_str().parse::<isize>().unwrap()).collect();

        let mut histories: Vec<Vec<isize>> = vec![vec![]];

        for i in 0..nums.len()-1 {
            histories[0].push(nums[i+1] - nums[i]);
        }

        let mut j = 0;
        loop {
            let mut new: Vec<isize> = vec![];
            for i in 0..histories[j].len()-1 {
               new.push(histories[j][i+1] - histories[j][i]);
            }

            histories.push(new);

            if histories[j+1].iter().sum::<isize>() == 0 {
                break;
            }

            j += 1;
        }
        println!("{:?}", histories);

        let mut next: isize = 0;
        for h in histories {
            next += h.last().unwrap();
        }
        next += nums.last().unwrap();

        println!("Next value: {}", next);
        solution += next;
    }

    solution
}

pub fn solve2(input: &String) -> isize {
    let mut solution = 0;

    let lines = input.lines();
    let nums_re = Regex::new(r"-?\d+").unwrap();

    for line in lines {
        let nums: Vec<isize> = nums_re.find_iter(line).map(|m| m.as_str().parse::<isize>().unwrap()).collect();

        let mut histories: Vec<Vec<isize>> = vec![vec![]];

        for i in 0..nums.len()-1 {
            histories[0].push(nums[i+1] - nums[i]);
        }

        let mut j = 0;
        loop {
            let mut new: Vec<isize> = vec![];
            for i in 0..histories[j].len()-1 {
               new.push(histories[j][i+1] - histories[j][i]);
            }

            histories.push(new);

            if histories[j+1].iter().sum::<isize>() == 0 {
                break;
            }

            j += 1;
        }
        println!("{:?}", histories);

        let mut next: isize = 0;
        for h in histories.iter().rev() {
            next = h.first().unwrap() - next;
            println!("De-extrapolating {} from value {}", next, h.first().unwrap());
        }
        next = nums.first().unwrap() - next;

        println!("Next value: {}", next);
        solution += next;
    }

    solution
}