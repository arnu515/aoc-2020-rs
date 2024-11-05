mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let mut args = std::env::args();
    let prog = args.next().unwrap();
    let args = args.take(2).collect::<Vec<String>>();
    if args.len() != 2 {
        panic!("Usage: {prog} <DAY_NUMBER> <PART_NUMBER>");
    }
    let args = args
        .iter()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>();
    let args = args.as_slice();

    match args {
        [1, part] => day1::solve(*part).unwrap(),
        [2, part] => day2::solve(*part).unwrap(),
        [3, part] => day3::solve(*part).unwrap(),
        [4, part] => day4::solve(*part).unwrap(),
        [5, part] => day5::solve(*part).unwrap(),
        _ => panic!("Invalid day/part"),
    };
}
