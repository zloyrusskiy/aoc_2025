use std::io;

fn main() {
    let input = parse_input();
    println!("input: {input:?}");

    let part1_res = calc_part1(&input);
    println!("part1 res {part1_res:?}");

    let part2_res = calc_part2(&input);
    println!("part2 res {part2_res:?}");
}

fn parse_input() -> Vec<(u64, u64)> {
    io::stdin()
        .lines()
        .map(|line| {
            let line_val = line.expect("Unable to read line");
            line_val
                .split(",")
                .filter(|range_str| !range_str.is_empty())
                .map(|range_str| {
                    let (from, to) = range_str.split_once('-').unwrap();
                    (from.parse::<u64>().unwrap(), to.parse::<u64>().unwrap())
                })
                .collect::<Vec<(u64, u64)>>()
        })
        .flatten()
        .collect()
}

fn calc_part1(input: &Vec<(u64, u64)>) -> u64 {
    let mut invalid_ids_sum: u64 = 0;
    for &(from, to) in input {
        for n in from..=to {
            let pow = n.ilog10();
            let pow_half = (pow + 1) / 2;
            let pow_factor = 10u64.pow(pow_half);
            if n % pow_factor == n / pow_factor {
                invalid_ids_sum += n;
            }
        }
    }

    invalid_ids_sum
}

fn calc_part2(input: &Vec<(u64, u64)>) -> u64 {
    let mut invalid_ids_sum: u64 = 0;
    for &(from, to) in input {
        for n in from..=to {
            let n_str = n.to_string();
            let invalid_id = (1..=(n_str.len() / 2))
                .filter(|&pat_len| n_str.len() % pat_len == 0)
                .find(|&pat_len| {
                    n_str.matches(&n_str[0..pat_len]).count() == n_str.len() / pat_len
                });

            if invalid_id.is_some() {
                invalid_ids_sum += n;
            }
        }
    }

    invalid_ids_sum
}
