use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn process(input: &str) -> i32 {
    let (path, first) = input
        .lines()
        .enumerate()
        .flat_map(|(y, s)| {
            s.chars()
                .enumerate()
                .filter(|(_, c)| !c.is_whitespace())
                .map(|(x, c)| ((x, y), c))
                .collect_vec()
        })
        .fold(
            (HashMap::new(), (1000i32, 1000i32)),
            |(mut path, mut first), ((x, y), c)| {
                path.insert((x as i32, y as i32), c);

                if c == '|' && y == 0 {
                    first = (x as i32, y as i32);
                }

                (path, first)
            },
        );

    let mut steps = 0;
    let mut c = path.get(&first);
    let mut cpos = first;
    let mut dx = 0;
    let mut dy = 1;

    while c.is_some() {
        let unwrapped = c.unwrap();

        match unwrapped {
            '+' => {
                let neighbours = [(0, 1), (1, 0), (-1, 0), (0, -1)]
                    .iter()
                    .filter(|(nx, ny)| !(nx == &(-dx) && ny == &(-dy)))
                    .collect_vec();

                for (nx, ny) in neighbours {
                    let n = path.get(&(cpos.0 + nx, cpos.1 + ny));
                    if n.is_some() {
                        dx = *nx;
                        dy = *ny;
                    }
                }
            }
            _ => {}
        }

        let next = (cpos.0 + dx, cpos.1 + dy);
        c = path.get(&next);
        cpos = next;
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
