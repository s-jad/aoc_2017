use itertools::Itertools;
use std::time::Instant;

fn process(input: usize) -> usize {
    let mut next_to_0 = None;
    let mut next_position = 0;

    for i in 1..=50_000_000 {
        next_position = (next_position + input) % i;
        if next_position == 0 {
            next_to_0 = Some(i);
        }
        next_position += 1;
    }

    next_to_0.unwrap()
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
