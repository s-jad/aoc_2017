use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let nums = input.chars().filter_map(|c| c.to_digit(10)).collect_vec();
    let first = nums[0];
    let last = nums[nums.len() - 1];

    let mut total = 0;

    if first == last {
        total += first as usize;
    }

    for (n1, n2) in nums.into_iter().tuple_windows::<(_, _)>() {
        if n1 == n2 {
            total += n2 as usize;
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
