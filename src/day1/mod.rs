use anyhow::Result;

mod part1 {
    fn find_nums(nums: Vec<usize>) -> Option<(usize, usize)> {
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i != j && nums[i] + nums[j] == 2020 {
                    return Some((nums[i], nums[j]));
                }
            }
        }
        None
    }

    pub fn solve(nums: Vec<usize>) {
        let (a, b) = find_nums(nums).unwrap();
        println!("{}", a * b);
    }
}

mod part2 {
    fn find_nums(nums: Vec<usize>) -> Option<(usize, usize, usize)> {
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                for k in 0..nums.len() {
                    if i != j && j != k && k != i && nums[i] + nums[j] + nums[k] == 2020 {
                        return Some((nums[i], nums[j], nums[k]));
                    }
                }
            }
        }
        None
    }

    pub fn solve(nums: Vec<usize>) {
        let (a, b, c) = find_nums(nums).unwrap();
        println!("{}", a * b * c);
    }
}

pub fn solve(part: u32) -> Result<()> {
    let nums: Vec<usize> = std::fs::read_to_string("./src/day1/in")?
        .lines()
        .map(|x| str::parse::<usize>(x).unwrap())
        .collect();

    match part {
        1 => part1::solve(nums),
        2 => part2::solve(nums),
        _ => panic!("Invalid part"),
    }
    Ok(())
}
