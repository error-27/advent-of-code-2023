pub fn solve(input: &String) -> isize {
    let lines = input.lines();
    let mut nums: Vec<isize> = Vec::new();

    for line in lines {
        let chars = line.chars();
        let mut digits: [char; 2] = [' ',' '];
        for c in chars {
            if c.is_numeric() {
                if digits[0] == ' ' {
                    digits[0] = c;
                }else{
                    digits[1] = c;
                }
            }
        }
        if digits[1] == ' ' {digits[1] = digits[0]}

        let d1: isize = digits[0].to_digit(10).unwrap() as isize;
        let d2: isize = digits[1].to_digit(10).unwrap() as isize;

        nums.push((d1*10) + d2);
    }

    nums.iter().map(|&i| i as isize).sum()
}

pub fn solve2(input: &String) -> isize {
    let lines = input.lines();
    let mut nums: Vec<isize> = Vec::new();
    let numwords = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for line in lines {
        let mut digit_indices: Vec<isize> = Vec::new();
        let mut digits: [isize; 2] = [0, 0];
        
        let chars = line.chars();
        for (pos, c) in chars.enumerate() {
            if c.is_numeric() {
                digit_indices.push(pos as isize);
            }
        }

        for word in numwords {
            let i = line.find(word);
            let j = line.rfind(word);

            if i.is_none() || j.is_none() {
                continue;
            }

            digit_indices.push(i.unwrap() as isize);
            digit_indices.push(j.unwrap() as isize);
        }
        digit_indices.sort();

        let first = digit_indices.first().unwrap();
        let last = digit_indices.last().unwrap();

        if line.chars().nth(*first as usize).unwrap().is_numeric() {
            digits[0] = line.chars().nth(*first as usize).unwrap().to_digit(10).unwrap() as isize; // stupid long line lmao
        }else{
            for (pos, w) in numwords.iter().enumerate() {
                let i = line.find(w);

                if i.is_none() {continue;}

                if i.unwrap() == *first as usize {
                    digits[0] = pos as isize + 1;
                    break;
                }
            }
        }

        if line.chars().nth(*last as usize).unwrap().is_numeric() {
            digits[1] = line.chars().nth(*last as usize).unwrap().to_digit(10).unwrap() as isize; // stupid long line lmao
        }else{
            for (pos, w) in numwords.iter().enumerate() {
                let i = line.rfind(w);

                if i.is_none() {continue;}

                if i.unwrap() == *last as usize {
                    digits[1] = pos as isize + 1;
                    break;
                }
            }
        }

        nums.push(10*digits[0] + digits[1]);
    }

    nums.iter().map(|&i| i as isize).sum()
}