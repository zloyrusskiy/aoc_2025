use std::io;

fn main() {
    let input = parse_input();
    println!("input: {input:?}");

    let part1_res = calc_part1(&input);
    println!("part1 res {part1_res:?}");

    let part2_res = calc_part2(&input);
    println!("part2 res {part2_res:?}");
}

fn parse_input() -> Vec<i32> {
    io::stdin()
        .lines()
        .map(|line| {
            let line_val = line.expect("Unable to read line");
            match line_val.chars().nth(0).unwrap() {
                'L' => -(line_val[1..].parse::<i32>().unwrap()),
                'R' => line_val[1..].parse::<i32>().unwrap(),
                _ => panic!("Invalid input {:?}", line_val),
            }
        })
        .collect()
}

fn calc_part1(input: &Vec<i32>) -> u64 {
    let mut cur = 50;
    let mut zero_seen = 0;

    for rot in input {
        cur += rot;
        cur %= 100;
        if cur < 0 {
            cur = 100 + cur;
        }
        if cur == 0 {
            zero_seen += 1;
        }
    }

    zero_seen as u64
}

fn calc_part2(input: &Vec<i32>) -> u64 {
    let mut cur = 50;
    let mut zero_seen = 0;
    for &rot in input {
        if rot == 0 {
            continue;
        }

        let dist = rot.abs();

        let steps_to_zero =
            if cur == 0 {
                100
            } else {
                if rot > 0 {100 - cur} else {cur}
            }
        ;

        if dist >= steps_to_zero {
            zero_seen += 1 + (dist - steps_to_zero) / 100;
        }

        cur = ((cur + rot) % 100 + 100) % 100;
    }

    zero_seen as u64
}

#[cfg(test)]
mod tests {
    use super::calc_part2;

    #[test]
    fn part2_empty_input() {
        let input: Vec<i32> = vec![];
        assert_eq!(calc_part2(&input), 0);
    }

    #[test]
    fn part2_simple_no_hit() {
        let input = vec![49];
        assert_eq!(calc_part2(&input), 0);
    }

    #[test]
    fn part2_multiple_hits() {
        let input = vec![150];
        assert_eq!(calc_part2(&input), 2);
    }

    #[test]
    fn part2_left_overflow_small() {
        let input = vec![-50, -1];
        assert_eq!(calc_part2(&input), 1);
    }

    #[test]
    fn part2_example_from_input1() {
        let input = vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82];
        assert_eq!(calc_part2(&input), 6);
    }
}
