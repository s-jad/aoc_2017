use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

fn process(input: &str) -> usize {
    const MEM_BANKS: usize = 16;
    let mut banks = [4, 1, 15, 12, 0, 9, 9, 5, 5, 8, 7, 3, 14, 5, 12, 3];

    let mut seen = HashSet::new();
    let mut steps = 0;

    while seen.insert(banks) {
        let (mut idx, max) = banks
            .iter()
            .enumerate()
            .fold((0, 0), |(a_idx, a_val), (idx, val)| {
                if *val > a_val {
                    (idx, *val)
                } else if a_val == *val {
                    if idx < a_idx {
                        (idx, *val)
                    } else {
                        (a_idx, a_val)
                    }
                } else {
                    (a_idx, a_val)
                }
            });

        banks[idx] = 0;

        for _ in 0..max {
            idx = (idx + 1) % MEM_BANKS;
            banks[idx] += 1;
        }
        steps += 1;
    }

    steps
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
