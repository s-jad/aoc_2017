use itertools::Itertools;
use std::{collections::VecDeque, time::Instant};

fn process(input: &str) -> String {
    let mut inputs = input
        .split_whitespace()
        .collect::<String>()
        .chars()
        .map(|c| c as u8)
        .collect_vec();

    let mut pepper: Vec<u8> = vec![17, 31, 73, 47, 23];

    inputs.append(&mut pepper);

    const LIST_SIZE: usize = 256;
    let mut list: VecDeque<usize> = VecDeque::with_capacity(LIST_SIZE);

    for i in 0..LIST_SIZE {
        list.push_back(i);
    }
    let mut skip_size = 0;
    let mut current_pos = 0;

    for i in 1..65 {
        for ip in &inputs {
            current_pos = (current_pos + skip_size) % LIST_SIZE;
            let mut temp = VecDeque::new();
            let idx = *ip;
            for _ in 0..idx {
                temp.push_back(list.pop_front().unwrap());
            }

            for _ in 0..idx {
                current_pos = (current_pos + 1) % LIST_SIZE;
                list.push_back(temp.pop_back().unwrap());
            }

            for _ in 0..skip_size {
                let f = list.pop_front().unwrap();
                list.push_back(f);
            }

            skip_size += 1;
        }
    }
    for _ in 0..current_pos {
        let f = list.pop_back().unwrap();
        list.push_front(f);
    }

    let sparse_hash = list.into_iter().enumerate().fold(
        (Vec::with_capacity(16), 0, None),
        |(mut sparse, mut s_idx, mut total), (l_idx, b)| {
            if total.is_none() {
                total = Some(b as u8);
            } else {
                total = Some(total.unwrap() ^ (b as u8));
            }
            if (l_idx + 1) % 16 == 0 {
                s_idx += 1;
                sparse.push(total.unwrap());
                total = None;
            }

            (sparse, s_idx, total)
        },
    );

    let mut hex_str = String::new();

    for num in sparse_hash.0.into_iter() {
        hex_str += &hex::encode(&[num]);
    }

    hex_str
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
