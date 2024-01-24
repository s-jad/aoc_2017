use itertools::Itertools;
use std::time::Instant;

fn process(input: usize) -> usize {
    let mut nums = Vec::with_capacity(2018);
    nums.push(0);

    let mut next_position = 0;
    for i in 1..=2017 {
        next_position = (next_position + input) % i;
        nums.insert(next_position, i);
        next_position += 1;
    }

    next_position = next_position % 2017;

    nums[next_position]
}

fn main() {
    let input = 301;

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
