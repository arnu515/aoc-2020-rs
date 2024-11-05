mod part1 {
    pub fn solve(input: &str) -> anyhow::Result<()> {
        todo!()
    }
}

pub fn solve(part: u32) -> anyhow::Result<()> {
    let input = include_str!("in");
    match part {
        1 => part1::solve(input),
        _ => Err(anyhow::anyhow!("Invalid part")),
    }
}
