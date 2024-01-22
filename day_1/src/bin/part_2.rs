use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let nums = input.chars().filter_map(|c| c.to_digit(10)).collect_vec();
    let len = nums.len();
    let mid = len / 2;
    let mut total = 0;

    for i in 0..nums.len() {
        let n1 = nums[i];
        let n2 = nums[(i + mid) % len];

        if n1 == n2 {
            total += n1 as usize;
        }
    }

    total
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
