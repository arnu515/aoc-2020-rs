mod part1 {
    use std::collections::HashSet;

    pub fn solve(input: &str) -> anyhow::Result<()> {
        println!(
            "{:?}",
            input
                .split("\n\n")
                .map(|x| {
                    let mut set: HashSet<char> = HashSet::new();
                    let mut answers = x.lines();
                    answers.next().unwrap().chars().for_each(|x| {
                        set.insert(x);
                    });
                    answers.for_each(|x| {
                        let mut new_set: HashSet<char> = HashSet::new();
                        x.chars().for_each(|x| {
                            new_set.insert(x);
                        });
                        set = set.intersection(&new_set).copied().collect::<HashSet<_>>();
                    });
                    set.len()
                })
                .sum::<usize>()
        );
        Ok(())
    }
}

pub fn solve(part: u32) -> anyhow::Result<()> {
    let input = include_str!("in");
    match part {
        1 => part1::solve(input),
        _ => Err(anyhow::anyhow!("Invalid part")),
    }
}
