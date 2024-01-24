use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let a_factor = 16807;
    let b_factor = 48271;
    let divisor = 2147483647;

    let (a_start, b_start) = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .find_map(|s| s.parse::<u64>().ok())
                .unwrap()
        })
        .collect_tuple::<(_, _)>()
        .unwrap();

    let mut a = a_start;
    let mut b = b_start;

    let mask: u64 = 0xFFFF;
    let mut matches = 0;
    let rounds = 5_000_000;

    for i in 0..rounds {
        while a % 4 != 0 {
            a = (a * a_factor) % divisor;
        }

        while b % 8 != 0 {
            b = (b * b_factor) % divisor;
        }

        let a_masked = a & mask;
        let b_masked = b & mask;

        if a_masked == b_masked {
            matches += 1;
        }

        a = (a * a_factor) % divisor;
        b = (b * b_factor) % divisor;
    }
    matches
}

fn main() {
    let input = include_str!("../../input.txt");

    let start = Instant::now();
    let output = process(input);
    let time = start.elapsed();

    dbg!(output, time);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("../../test.txt");
        let output = process(input);
        assert_eq!(result,);
    }
}
