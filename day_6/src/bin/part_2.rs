use itertools::Itertools;
use std::{
    collections::{hash_map::Entry, HashMap},
    time::Instant,
};

fn process() -> usize {
    const MEM_BANKS: usize = 16;
    let mut banks = [4, 1, 15, 12, 0, 9, 9, 5, 5, 8, 7, 3, 14, 5, 12, 3];

    let mut seen: HashMap<[i32; MEM_BANKS], usize> = HashMap::new();
    let mut steps = 0;
    let mut found_loop = false;
    let mut prev_step = None;

    while !found_loop {
        let entry = seen.entry(banks);

        prev_step = match entry {
            Entry::Occupied(mut o) => {
                found_loop = true;
                Some(*o.get_mut())
            }
            Entry::Vacant(v) => {
                v.insert(steps);
                None
            }
        };

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

    steps - prev_step.unwrap() - 1
}

fn main() {
    let start = Instant::now();
    let output = process();
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
