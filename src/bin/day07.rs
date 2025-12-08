use std::io;

#[derive(Debug)]
struct Input {
    lines: Vec<Vec<char>>,
}

impl From<Vec<Vec<char>>> for Input {
    fn from(lines: Vec<Vec<char>>) -> Self {
        Self { lines }
    }
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
    io::stdin()
        .lines()
        .map(|line| {
            let line_val = line.expect("Unable to read line");
            line_val.chars().collect()
        })
        .collect::<Vec<_>>()
        .into()
}

fn calc_part1(input: &Input) -> u64 {
    let cols = input.lines[0].len();
    let mut beams: Vec<bool> = vec![false; cols];
    let mut split_count = 0;
    for ci in 0..cols {
        if input.lines[0][ci] == 'S' {
            beams[ci] = true;
        }
    }
    for r in input.lines.iter().skip(1) {
        for c in 0..cols {
            if r[c] == '^' && beams[c] {
                split_count += 1;

                beams[c] = false;
                if c > 0 {
                    beams[c - 1] = true;
                }

                if c < cols - 1 {
                    beams[c + 1] = true;
                }
            }
        }
    }

    split_count
}

fn calc_part2(input: &Input) -> u64 {
    let cols = input.lines[0].len();
    let mut beams: Vec<u64> = vec![0; cols];
    for ci in 0..cols {
        if input.lines[0][ci] == 'S' {
            beams[ci] = 1;
        }
    }
    for r in input.lines.iter().skip(1) {
        for c in 0..cols {
            if r[c] == '^' && beams[c] > 0 {
                if c > 0 {
                    beams[c - 1] += beams[c];
                }

                if c < cols - 1 {
                    beams[c + 1] += beams[c];
                }

                beams[c] = 0;
            }
        }
    }

    beams.iter().sum()
}
