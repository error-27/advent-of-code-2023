pub fn solve(input: String) -> usize {
    let lines = input.lines();
    let mut nums: Vec<usize> = Vec::new();

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
        let d1: usize = digits[0].to_digit(10).unwrap() as usize;
        let d2: usize = digits[1].to_digit(10).unwrap() as usize;

        nums.push((d1*10) + d2);
    }

    nums.iter().map(|&i| i as usize).sum()
}