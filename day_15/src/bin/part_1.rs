use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

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

    let mut a_step = a_start;
    let mut b_step = b_start;

    let mask: u64 = 0xFFFF;
    let mut matches = 0;

    for _ in 0..40_000_000 {
        a_step = (a_step * a_factor) % divisor;
        b_step = (b_step * b_factor) % divisor;

        let a_masked = a_step & mask;
        let b_masked = b_step & mask;

        if a_masked == b_masked {
            matches += 1;
        }
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
