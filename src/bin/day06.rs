use std::io;
use fancy_regex::Regex;

#[derive(Debug)]
struct Input {
    nums: Vec<Vec<String>>,
    ops: Vec<char>,
}


fn main() {
    let input = parse_input();
    println!("input: {input:?}");

    let part1_res = calc_part1(&input);
    println!("part1 res {part1_res:?}");

    let part2_res = calc_part2(&input);
    println!("part2 res {part2_res:?}");
}

fn parse_input() -> Input {
    let re = Regex::new(r"\s(?=[*+])").unwrap();
    let mut nums: Vec<Vec<String>> = vec![];
    let mut ops: Vec<char> = vec![];

    let lines: Vec<_> = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let ops_line = lines.iter().rev().next().unwrap();
    let split_ops = re.split(ops_line).map(|x| x.unwrap().to_string()).collect::<Vec<String>>();
    let ops = split_ops.iter().map(|op_str| op_str.chars().next().unwrap()).collect::<Vec<char>>();


    for i in 0..lines.len() - 1 {
        let line = lines[i].clone();
        let mut line_pos = 0;
        let mut row = vec![];
        for split_op in split_ops.iter() {
            let op_len = split_op.len();
            let num_str = &line[line_pos..line_pos + op_len];
            line_pos += op_len + 1;
            row.push(num_str.to_string())
        }
        nums.push(row);
    }

    Input { nums, ops }
}

fn calc_part1(input: &Input) -> u64 {
    input.nums
        .iter()
        .map(|num_arr|
            num_arr.iter()
                .map(|n| n.trim().parse::<u64>().unwrap())
                .collect()
        )
        .reduce(|acc: Vec<u64>, num_arr| {
            (0..num_arr.len())
                .into_iter()
                .map(|i| {
                    match input.ops[i] {
                        '*' => acc[i] * num_arr[i],
                        '+' => acc[i] + num_arr[i],
                        _ => panic!("i don't know that fucking operation: {} in row {i}", input.ops[i]),
                    }
                })
                .collect::<Vec<u64>>()
        })
        .unwrap()
        .into_iter()
        .sum::<u64>()
}

fn calc_part2(input: &Input) -> u64 {
    let mut transposed_nums: Vec<Vec<u64>> = vec![];
    let cols = input.nums[0].len();
    for c in 0..cols {
        let mut col_nums: Vec<u64> = vec![];
        for ci in 0..input.nums[0][c].len() {
            let mut new_num = String::new();
            for r in 0..input.nums.len() {
                new_num += &input.nums[r][c].get(ci..ci + 1).unwrap();
            }
            col_nums.push(new_num.trim().parse::<u64>().unwrap());
        }
        transposed_nums.push(col_nums);
    }

    let mut problems_calced: Vec<u64> = vec![];
    for i in 0..transposed_nums.len() {
        let t_row = transposed_nums[i].iter().cloned();
        let p_res = t_row.reduce(|acc, num| {
            match input.ops[i] {
                '*' => acc * num,
                '+' => acc + num,
                _ => panic!("i don't know that fucking operation: {} in row {i}", input.ops[i]),
            }
        }).unwrap();
        problems_calced.push(p_res.clone());
    }

    problems_calced
        .into_iter()
        .sum::<u64>()
}
