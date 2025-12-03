use std::io;

fn main() {
    let input = parse_input();
    println!("input: {input:?}");

    let part1_res = calc_part1(&input);
    println!("part1 res {part1_res:?}");

    let part2_res = calc_part2(&input);
    println!("part2 res {part2_res:?}");
}

fn parse_input() -> Vec<Vec<u64>> {
    io::stdin()
        .lines()
        .map(|line| {
            let line_val = line.expect("Unable to read line");
            line_val
                .chars()
                .map(|ch| ch.to_digit(10).unwrap() as u64)
                .collect::<Vec<u64>>()
        })
        .collect()
}

fn calc_part1(input: &Vec<Vec<u64>>) -> u64 {
    input
        .iter()
        .map(|bat_vals| {
            let (fb_ind, fb_val) = &bat_vals[0..bat_vals.len() - 1]
                .iter()
                .enumerate()
                .rev()
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap();
            let sb_val = &bat_vals[fb_ind + 1..].iter().max().unwrap();

            *fb_val * 10 + *sb_val
        })
        .sum::<u64>()
}

fn calc_part2(input: &Vec<Vec<u64>>) -> u64 {
    input
        .iter()
        .map(|bat_vals| {
            let mut final_vals: Vec<u64> = Vec::with_capacity(12);
            let mut left_border = 0;

            for bn in (1..=12).rev() {
                let (b_ind, b_val) = &bat_vals[left_border..=bat_vals.len() - bn]
                    .iter()
                    .enumerate()
                    .rev()
                    .max_by(|(_, a), (_, b)| a.cmp(b))
                    .unwrap();

                final_vals.push(**b_val);
                left_border = left_border + b_ind + 1;
            }

            let mut res = 0;
            for val in final_vals {
                res = res * 10 + val;
            }

            res
        })
        .sum::<u64>()

}
