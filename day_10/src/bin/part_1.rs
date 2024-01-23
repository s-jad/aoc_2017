use itertools::Itertools;
use std::{collections::VecDeque, time::Instant};

fn process(input: &str) -> usize {
    let inputs = input
        .split_terminator(&[' ', ',', '\n'][..])
        .into_iter()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect_vec();

    const LIST_SIZE: usize = 256;
    let mut list = VecDeque::with_capacity(LIST_SIZE);

    for i in 0..LIST_SIZE {
        list.push_back(i);
    }
    let mut skip_size = 0;
    let mut current_pos = 0;

    for ip in inputs {
        current_pos = (current_pos + skip_size) % LIST_SIZE;
        let mut temp = VecDeque::new();
        for _ in 0..ip {
            temp.push_back(list.pop_front().unwrap());
        }

        for _ in 0..ip {
            current_pos = (current_pos + 1) % LIST_SIZE;
            list.push_back(temp.pop_back().unwrap());
        }

        for _ in 0..skip_size {
            let f = list.pop_front().unwrap();
            list.push_back(f);
        }

        skip_size += 1;
    }

    for _ in 0..current_pos {
        let f = list.pop_back().unwrap();
        list.push_front(f);
    }

    list[0] * list[1]
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
