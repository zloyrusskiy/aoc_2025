use std::io;

const DIFFS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn main() {
    let input = parse_input();
    println!("input: {input:?}");

    let part1_res = calc_part1(&input);
    println!("part1 res {part1_res:?}");

    let part2_res = calc_part2(&input);
    println!("part2 res {part2_res:?}");
}

fn parse_input() -> Vec<Vec<char>> {
    io::stdin()
        .lines()
        .map(|line| {
            let line_val = line.expect("Unable to read line");
            line_val.chars().collect()
        })
        .collect()
}

fn calc_part1(input: &Vec<Vec<char>>) -> u64 {
    let mut accessible_qty = 0;

    for ri in 0..input.len() {
        for ci in 0..input[0].len() {
            if input[ri][ci] == '@' {
                if get_neighbours(ri, ci, input).iter().count() < 4 {
                    accessible_qty += 1;
                }
            }
        }
    }

    accessible_qty
}

fn get_neighbours(r: usize, c: usize, input: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    for (dr, dc) in DIFFS {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr >= 0
            && nr < input.len() as i32
            && nc >= 0
            && nc < input[0].len() as i32
            && input[nr as usize][nc as usize] == '@'
        {
            neighbours.push((nr as usize, nc as usize));
        }
    }
    neighbours
}

fn calc_part2(input: &Vec<Vec<char>>) -> u64 {
    let mut cur_snapshot = input.clone();
    let mut total_removed = 0;

    loop {
        let mut to_be_removed = Vec::new();

        for ri in 0..cur_snapshot.len() {
            for ci in 0..cur_snapshot[0].len() {
                if cur_snapshot[ri][ci] == '@' {
                    if get_neighbours(ri, ci, &cur_snapshot).iter().count() < 4 {
                        to_be_removed.push((ri, ci));
                    }
                }
            }
        }

        if to_be_removed.len() == 0 {
            break;
        }

        total_removed += to_be_removed.len() as u64;
        for (r, c) in to_be_removed {
            cur_snapshot[r][c] = '.';
        }
    }

    total_removed
}
