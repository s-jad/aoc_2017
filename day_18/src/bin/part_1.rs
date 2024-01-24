use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

#[derive(Debug)]
struct ISet<'a> {
    op: &'a str,
    v1: &'a str,
    v2: Option<&'a str>,
}

fn parse_instructions<'a>(
    registers: &mut HashMap<&'a str, isize>,
    ins_vec: Vec<ISet<'a>>,
    last_freq: &mut isize,
    last_recorded: &mut isize,
) {
    let mut found = false;
    let mut idx = 0;
    while !found {
        let instruction = &ins_vec[idx];
        registers.entry(instruction.v1).or_insert(0);
        let m = instruction.v2;

        let mut n;
        if !m.is_some() {
            n = registers[&instruction.v1];
        } else {
            let m2 = m.unwrap();
            let m_num = match m2.parse::<isize>() {
                Ok(n) => Some(n),
                ParseIntError => None,
            };

            if m_num.is_some() {
                n = m_num.unwrap();
            } else {
                n = registers[&m2];
            }
        }

        match instruction.op {
            "add" => {
                let entry = registers.get_mut(instruction.v1).unwrap();
                *entry = *entry + n;

                idx += 1;
            }
            "mul" => {
                let entry = registers.get_mut(instruction.v1).unwrap();
                *entry = *entry * n;

                idx += 1;
            }
            "mod" => {
                let entry = registers.get_mut(instruction.v1).unwrap();
                *entry = *entry % n;

                idx += 1;
            }
            "set" => {
                let entry = registers.get_mut(instruction.v1).unwrap();
                *entry = n;

                idx += 1;
            }
            "snd" => {
                *last_freq = registers[&instruction.v1];

                idx += 1;
            }
            "rcv" => {
                if registers[&instruction.v1] > 0 {
                    *last_recorded = *last_freq;
                    found = true;
                }

                idx += 1;
            }
            "jgz" => {
                if registers[&instruction.v1] > 0 {
                    idx = (idx as isize + n) as usize;
                } else {
                    idx += 1;
                }
            }
            _ => unreachable!(),
        }
    }
}

fn process(input: &str) -> isize {
    let ins = input
        .lines()
        .map(|l| {
            let parts = l.split_whitespace().collect_vec();
            let (op, v1) = (parts[0], parts[1]);
            let mut v2 = None;
            if parts.len() == 3 {
                v2 = Some(parts[2]);
            }
            ISet { op, v1, v2 }
        })
        .collect_vec();

    let mut registers = HashMap::new();
    let mut last_freq = 0;
    let mut last_recorded = 0;

    parse_instructions(&mut registers, ins, &mut last_freq, &mut last_recorded);

    last_recorded
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
