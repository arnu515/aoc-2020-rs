fn get_id(str: &str) -> usize {
    let row = {
        let mut l_row = 0;
        let mut r_row = 127;
        for char in str[..7].chars() {
            match char {
                'F' => r_row = (r_row + l_row) / 2,
                'B' => l_row = (r_row + l_row) / 2 + 1,
                _ => unreachable!(),
            };
        }
        l_row
    };
    let col = {
        let mut l_col = 0;
        let mut r_col = 7;
        for char in str[7..].chars() {
            match char {
                'L' => r_col = (r_col + l_col) / 2,
                'R' => l_col = (r_col + l_col) / 2 + 1,
                _ => unreachable!(),
            };
        }
        l_col
    };
    row * 8 + col
}

mod part1 {
    pub fn solve(input: &str) -> anyhow::Result<()> {
        println!("{}", input.lines().map(super::get_id).max().unwrap());
        Ok(())
    }
}

mod part2 {
    pub fn solve(input: &str) -> anyhow::Result<()> {
        let mut iter = input.lines().map(super::get_id).collect::<Vec<_>>();
        iter.sort();
        let mut iter = iter.iter();
        let mut prev_el = *iter.next().unwrap();
        let seat = iter
            .skip_while(|x| {
                let res = **x == prev_el + 1;
                prev_el = **x;
                res
            })
            .next()
            .copied()
            .unwrap()
            - 1;
        println!("{}", seat);
        Ok(())
    }
}

pub fn solve(part: u32) -> anyhow::Result<()> {
    let input = include_str!("in");
    match part {
        1 => part1::solve(input),
        2 => part2::solve(input),
        _ => Err(anyhow::anyhow!("Invalid part")),
    }
}
