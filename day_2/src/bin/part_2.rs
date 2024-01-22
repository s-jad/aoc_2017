use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| {
            let nums = l
                .split_whitespace()
                .into_iter()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_vec();

            let divisible = nums
                .into_iter()
                .combinations(2)
                .filter(|v| v[0] != v[1])
                .find(|v| v[0].max(v[1]) % v[0].min(v[1]) == 0);

            divisible
        })
        .map(|d| d[0].max(d[1]) / d[0].min(d[1]))
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
