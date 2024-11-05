use anyhow::Result;

mod part1 {
    use anyhow::{anyhow, Result};

    fn solve_line(line: &str) -> Result<bool> {
        let (nums, rest) = line.split_once(' ').ok_or(anyhow!("Invalid input"))?;
        let (min, max) = nums.split_once('-').ok_or(anyhow!("Invalid input"))?;
        let min: usize = min.parse()?;
        let max: usize = max.parse()?;
        let mut rest = rest.chars();
        let letter = rest.next().ok_or(anyhow!("Invalid input"))?;
        let count = rest.skip(2).filter(|x| *x == letter).count();
        Ok(count >= min && count <= max)
    }

    pub fn solve(input: &str) -> Result<()> {
        println!(
            "{}",
            input.lines().filter(|x| solve_line(*x).unwrap()).count()
        );
        Ok(())
    }
}

mod part2 {
    use anyhow::{anyhow, Result};

    fn solve_line(line: &str) -> Result<bool> {
        let (nums, rest) = line.split_once(' ').ok_or(anyhow!("Invalid input"))?;
        let (pos1, pos2) = {
            let (min, max) = nums.split_once('-').ok_or(anyhow!("Invalid input"))?;
            (min.parse::<usize>()?, max.parse::<usize>()?)
        };
        let mut rest = rest.chars();
        let letter = rest.next().ok_or(anyhow!("Invalid input"))?;
        let l_pos1 = rest.nth(pos1 + 1).ok_or(anyhow!("Invalid input"))?;
        let l_pos2 = rest.nth(pos2 - pos1 - 1).ok_or(anyhow!("Invalid input"))?;
        println!("{pos1}: {l_pos1} ;; {pos2}: {l_pos2}");
        Ok(l_pos1 == letter && l_pos2 != letter || l_pos1 != letter && l_pos2 == letter)
    }

    pub fn solve(input: &str) -> Result<()> {
        println!(
            "{}",
            input.lines().filter(|&x| solve_line(x).unwrap()).count()
        );
        Ok(())
    }
}

pub fn solve(part: u32) -> Result<()> {
    let input = std::fs::read_to_string("./src/day2/in")?;

    match part {
        1 => part1::solve(&input),
        2 => part2::solve(&input),
        _ => panic!("Invalid part"),
    }
}
