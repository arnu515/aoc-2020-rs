mod part1 {
    pub fn solve(input: &str) -> anyhow::Result<()> {
        let fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        println!(
            "{:?}",
            input
                .trim_end()
                .split("\n\n")
                .map(|x| x.replace('\n', " "))
                .map(|x| x
                    .split(' ')
                    .map(|x| x.split_once(':').unwrap().0)
                    .filter(|x| fields.contains(x))
                    .count())
                .filter(|&x| x == 7)
                .count()
        );
        Ok(())
    }
}

mod part2 {
    const ECL_ALLOWED: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    fn validate((key, val): (&str, &str)) -> bool {
        match key {
            "byr" => (1920..=2002).contains(&val.parse::<u16>().unwrap_or(0)),
            "iyr" => (2010..=2020).contains(&val.parse::<u16>().unwrap_or(0)),
            "eyr" => (2020..=2030).contains(&val.parse::<u16>().unwrap_or(0)),
            "hgt" => {
                let num = &val[..val.len() - 2].parse::<u8>().unwrap_or(0);
                match &val[val.len() - 2..] {
                    "cm" => (150..=193).contains(num),
                    "in" => (59..=76).contains(num),
                    _ => false,
                }
            }
            "hcl" => {
                let hexa = "0123456789abcdef";
                if val.len() != 7 || val.as_bytes()[0] != b'#' {
                    return false;
                }
                val.chars().skip(1).all(|x| hexa.contains(x))
            }
            "ecl" => ECL_ALLOWED.contains(&val),
            "pid" => val.len() == 9,
            _ => true,
        }
    }

    pub fn solve(input: &str) -> anyhow::Result<()> {
        println!(
            "{:?}",
            input
                .trim_end()
                .split("\n\n")
                .map(|x| x.replace('\n', " "))
                .map(|x| {
                    let iter = x
                        .split(' ')
                        .map(|x| x.split_once(':').unwrap())
                        .filter(|(x, _)| FIELDS.contains(x));
                    let mut count = 0;
                    let mut res = true;
                    for x in iter {
                        count += 1;
                        res = res && validate(x);
                    }
                    return (count == 7 || count == 8) && res;
                })
                .filter(|&x| x)
                .count() // .collect::<Vec<bool>>()
        );
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
