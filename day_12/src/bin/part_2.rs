use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn count_groups<'a>(
    pipes: &HashMap<&str, Vec<&'a str>>,
    mut seen: &mut HashSet<&'a str>,
    current: &'a str,
) {
    if !(*seen).insert(current) {
        return;
    } else {
        let children = pipes.get(current).unwrap();

        for child in children {
            count_groups(pipes, &mut seen, *child);
        }
    }
}

fn process(input: &str) -> usize {
    let mut pipes = input
        .lines()
        .map(|l| {
            l.split_terminator(&[' ', '\n', '<', '>', '-', ','])
                .filter(|s| !s.is_empty())
                .collect_vec()
        })
        .fold(HashMap::new(), |mut acc, parts| {
            acc.insert(parts[0], parts[1..].to_vec());
            acc
        });

    let mut seen = HashSet::new();

    let mut total = 0;
    let mut current = Some(&"0");
    while current.is_some() {
        count_groups(&pipes, &mut seen, current.unwrap());

        for s in seen.iter() {
            pipes.remove(s);
        }
        total += 1;

        current = pipes.keys().next();
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
