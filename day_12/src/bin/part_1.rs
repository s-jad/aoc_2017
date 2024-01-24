use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn count_paths_to_root<'a>(
    pipes: &HashMap<&str, Vec<&'a str>>,
    mut seen: &mut HashSet<&'a str>,
    current: &'a str,
) -> usize {
    if !(*seen).insert(current) {
        return 0;
    } else {
        let children = pipes.get(current).unwrap();

        let mut count = 1;
        for child in children {
            count += count_paths_to_root(pipes, &mut seen, *child);
        }
        return count;
    }
}

fn process(input: &str) -> usize {
    let pipes = input
        .lines()
        .map(|l| {
            l.split_terminator(&[' ', '\n', '<', '>', '-', ','])
                .filter(|s| !s.is_empty())
                .collect_vec()
        })
        .fold(HashMap::new(), |mut acc, parts| {
            if parts[0] != parts[1] {
                acc.insert(parts[0], parts[1..].to_vec());
            } else {
                acc.insert(parts[0], vec![]);
            }
            acc
        });

    let mut seen = HashSet::new();
    let total = count_paths_to_root(&pipes, &mut seen, "0");

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
