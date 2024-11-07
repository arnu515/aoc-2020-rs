mod part1 {
    use std::collections::HashMap;

    fn generate_hashmap(input: &str) -> HashMap<String, Vec<String>> {
        let mut hashsho_mappu: HashMap<String, Vec<String>> = HashMap::new();
        let lines = input.lines();
        lines.for_each(|x| {
            let (bag_name, contains) = x.split_once(" contain ").unwrap();
            let bag_name = bag_name[..bag_name.len() - 5].to_owned(); // remove " bags"
            contains[..contains.len() - 1]
                .split(',')
                .map(|x| {
                    let mut x = x.split_whitespace().skip(1).take(2);
                    format!("{} {}", x.next().unwrap(), x.next().unwrap())
                })
                .for_each(|x| {
                    if x == "other bags" {
                        hashsho_mappu.insert(bag_name.to_owned(), vec![]);
                        return;
                    }
                    match hashsho_mappu.get_mut(&bag_name) {
                        Some(v) => {
                            v.push(x.to_owned());
                        }
                        None => {
                            hashsho_mappu.insert(bag_name.to_owned(), vec![x.to_owned()]);
                        }
                    };
                });
        });
        hashsho_mappu
    }

    fn contains_shiny_gold(hashmap: &HashMap<String, Vec<String>>, k: &str) -> bool {
        hashmap
            .get(k)
            .unwrap()
            .iter()
            .any(|k| k == "shiny gold" || contains_shiny_gold(hashmap, k))
    }

    pub fn solve(input: &str) -> anyhow::Result<()> {
        let hashmap = generate_hashmap(input);
        println!(
            "{}",
            hashmap
                .keys()
                .filter(|&k| k != "shiny gold")
                .filter(|&k| contains_shiny_gold(&hashmap, &k))
                .count()
        );
        Ok(())
    }
}

mod part2 {
    use std::collections::HashMap;

    fn generate_hashmap(input: &str) -> HashMap<String, Vec<(usize, String)>> {
        let mut hashsho_mappu: HashMap<String, Vec<(usize, String)>> = HashMap::new();
        let lines = input.lines();
        lines.for_each(|x| {
            let (bag_name, contains) = x.split_once(" contain ").unwrap();
            let bag_name = bag_name[..bag_name.len() - 5].to_owned(); // remove " bags"
            contains[..contains.len() - 1]
                .split(',')
                .map(|x| {
                    let mut x = x.split_whitespace().take(3);
                    let num = x.next().unwrap();
                    if num == "no" {
                        (0, String::from("-"))
                    } else {
                        (
                            num.parse::<usize>().unwrap(),
                            format!("{} {}", x.next().unwrap(), x.next().unwrap()),
                        )
                    }
                })
                .for_each(|x| {
                    if x.1 == "-" {
                        hashsho_mappu.insert(bag_name.to_owned(), vec![]);
                        return;
                    }
                    match hashsho_mappu.get_mut(&bag_name) {
                        Some(v) => {
                            v.push(x);
                        }
                        None => {
                            hashsho_mappu.insert(bag_name.to_owned(), vec![x]);
                        }
                    };
                });
        });
        hashsho_mappu
    }

    fn find_num_bags(hashmap: &HashMap<String, Vec<(usize, String)>>, k: &str) -> usize {
        hashmap
            .get(k)
            .unwrap()
            .iter()
            .map(|x| x.0 + x.0 * find_num_bags(hashmap, &x.1))
            .sum()
    }

    pub fn solve(input: &str) -> anyhow::Result<()> {
        let hashmap = generate_hashmap(input);
        println!("{:?}", find_num_bags(&hashmap, "shiny gold"));
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
