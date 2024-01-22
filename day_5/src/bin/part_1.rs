use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let mut nums = input
        .lines()
        .filter_map(|l| l.trim().parse::<i32>().ok())
        .collect_vec();

    let mut idx = 0;
    let mut steps = 0;
    while idx < nums.len() {
        let jmp = nums[idx];
        nums[idx] += 1;
        idx = (idx as i32 + jmp) as usize;
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
