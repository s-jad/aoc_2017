use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

fn parse_instruction(ins: &str, programs: VecDeque<char>) -> VecDeque<char> {
    let mut np = programs;
    match ins.chars().next().unwrap() {
        's' => {
            let len = ins
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            for _ in 0..len {
                let temp = np.pop_back().unwrap();
                np.push_front(temp);
            }
        }
        'x' => {
            let (a_idx, b_idx) = ins
                .trim_start_matches("x")
                .split("/")
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_tuple::<(_, _)>()
                .unwrap();

            let temp = np[a_idx];
            np[a_idx] = np[b_idx];
            np[b_idx] = temp;
        }
        'p' => {
            let (a_c, b_c) = ins
                .chars()
                .skip(1)
                .filter(|c| c.is_alphabetic())
                .collect_tuple::<(_, _)>()
                .unwrap();

            let a_idx = np.iter().enumerate().find(|(_, c)| **c == a_c).unwrap().0;

            let b_idx = np.iter().enumerate().find(|(_, c)| **c == b_c).unwrap().0;

            let temp = np[a_idx];
            np[a_idx] = np[b_idx];
            np[b_idx] = temp;
        }
        _ => unreachable!(),
    }

    np
}

fn process(input: &str) -> String {
    let mut programs = "abcdefghijklmnop".chars().collect::<VecDeque<char>>();

    let instructions = input.split_terminator(&[',', '\n']).collect_vec();

    let mut seen = HashSet::new();
    let mut dances = HashMap::new();
    let mut idx = 0;

    dances.insert(idx, programs.iter().collect::<String>());

    while seen.insert(programs.iter().collect::<String>()) {
        for i in &instructions {
            programs = parse_instruction(i, programs);
        }
        idx += 1;
        dances.insert(idx, programs.iter().collect::<String>());
    }

    let loop_interval = idx;

    let end_dance = 1_000_000_000 % loop_interval;

    dances.get(&end_dance).unwrap().to_owned()
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
