use regex::Regex;

pub fn solve(input: &String) -> usize {
    let mut solution: usize = 0;
    let re_nums = Regex::new(r"\d+").unwrap();

    let seeds: Vec<usize> = re_nums.find_iter(
        input.lines().collect::<Vec<_>>().first().unwrap())
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .collect();
    
    println!("{:?}", seeds);
    
    let re_maps = vec![
        Regex::new(r"seed-to-soil map:\n((\d+ )(\d+ )(\d+)\n)+").unwrap(),
        Regex::new(r"soil-to-fertilizer map:\n((\d+ )(\d+ )(\d+)\n)+").unwrap(),
        Regex::new(r"fertilizer-to-water map:\n((\d+ )(\d+ )(\d+)\n)+").unwrap(),
        Regex::new(r"water-to-light map:\n((\d+ )(\d+ )(\d+)\n)+").unwrap(),
        Regex::new(r"light-to-temperature map:\n((\d+ )(\d+ )(\d+)\n)+").unwrap(),
        Regex::new(r"temperature-to-humidity map:\n((\d+ )(\d+ )(\d+)\n)+").unwrap(),
        Regex::new(r"humidity-to-location map:\n((\d+ )(\d+ )(\d+)\n)+").unwrap()
    ];

    let mut map_ranges: Vec<Vec<[usize; 3]>> = vec![];

    for r in re_maps {
        let text = r.find(input).unwrap().as_str();
        let map_nums: Vec<usize> = re_nums.find_iter(text).map(|i| i.as_str().parse::<usize>().unwrap()).collect();
        
        let mut map: Vec<[usize; 3]> = vec![];
        for i in 0..map_nums.len()/3 {
            map.push([map_nums[i*3], map_nums[i*3+1], map_nums[i*3+2]]);
        }

        //println!("{:?}", map);
        map_ranges.push(map);
    }

    solution = seeds[0];
    for seed in seeds {
        let mut value = seed;
        for r in map_ranges.iter() {
            let src_range: Vec<Vec<usize>> = r.iter().map(|i| (i[1]..i[1]+i[2]).collect()).collect();
            let dest_range: Vec<Vec<usize>> = r.iter().map(|i| (i[0]..i[0]+i[2]).collect()).collect();

            for i in 0..src_range.len() {
                if !src_range[i].contains(&value) { continue; }
                value = dest_range[i][src_range[i].binary_search(&value).unwrap()];
                //println!("value stage: {}", value);
            }
        }

        solution = solution.min(value);
        //println!("value: {}", value);
    }

    solution
}

pub fn solve2(input: &String) -> usize {
    0
}