use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let parts = l.split_whitespace().collect_vec();
            let p_set = parts
                .iter()
                .map(|part| {
                    let mut p = part.chars().collect_vec();
                    p.sort();
                    p
                })
                .collect::<HashSet<Vec<char>>>();

            if parts.len() == p_set.len() {
                1
            } else {
                0
            }
        })
        .sum()
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
