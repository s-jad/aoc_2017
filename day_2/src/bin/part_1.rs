use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .into_iter()
                .filter_map(|s| s.parse::<usize>().ok())
                .fold(
                    (0, std::usize::MIN, std::usize::MAX),
                    |(mut diff, mut max, mut min), n| {
                        if n > max {
                            max = n;
                        }
                        if n < min {
                            min = n;
                        }
                        diff = max - min;
                        (diff, max, min)
                    },
                )
                .0
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
