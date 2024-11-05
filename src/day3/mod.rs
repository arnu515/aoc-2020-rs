mod part1 {
    pub fn solve(input: &str) -> anyhow::Result<()> {
        let horiz_len = input.lines().nth(1).unwrap().len();
        let mut h = 0;

        println!(
            "{}",
            input
                .lines()
                .skip(1)
                .filter(|&x| {
                    h = (h + 3) % horiz_len;
                    x.as_bytes()[h] == b'#'
                })
                .count()
        );

        Ok(())
    }
}

mod part2 {
    pub fn solve(input: &str) -> anyhow::Result<()> {
        let horiz_len = input.lines().nth(1).unwrap().len();
        let mut h1 = 0;
        let mut h2 = 0;
        let mut h3 = 0;
        let mut h4 = 0;
        let mut h5 = 0;
        let mut down_2_flag = false;
        let mut count1: usize = 0;
        let mut count2: usize = 0;
        let mut count3: usize = 0;
        let mut count4: usize = 0;
        let mut count5: usize = 0;

        for line in input.lines().skip(1) {
            h1 = (h1 + 1) % horiz_len;
            if line.as_bytes()[h1] == b'#' {
                count1 += 1;
            }
            h2 = (h2 + 3) % horiz_len;
            if line.as_bytes()[h2] == b'#' {
                count2 += 1;
            }
            h3 = (h3 + 5) % horiz_len;
            if line.as_bytes()[h3] == b'#' {
                count3 += 1;
            }
            h4 = (h4 + 7) % horiz_len;
            if line.as_bytes()[h4] == b'#' {
                count4 += 1;
            }
            if down_2_flag {
                h5 = (h5 + 1) % horiz_len;
                if line.as_bytes()[h5] == b'#' {
                    count5 += 1;
                }
            }
            down_2_flag = !down_2_flag;
        }

        println!("{}", count1 * count2 * count3 * count4 * count5);

        Ok(())
    }
}

pub fn solve(part: u32) -> anyhow::Result<()> {
    let input = include_str!("in");
    match part {
        1 => part1::solve(input)?,
        2 => part2::solve(input)?,
        _ => panic!("Invalid part"),
    }

    Ok(())
}
