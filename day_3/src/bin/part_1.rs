use itertools::Itertools;
use std::time::Instant;

#[derive(Debug)]
struct Ring {
    no: usize,
    size: usize,
    biggest_num: usize,
}

impl Ring {
    fn new(no: usize, size: usize, biggest_num: usize) -> Self {
        Ring {
            no,
            size,
            biggest_num,
        }
    }
}

fn process(input: usize) -> usize {
    let ring_1 = 1;
    let ring_2 = ring_1 + 7;

    let mut current_ring = Ring::new(3, ring_2 + 8, 25);

    while current_ring.biggest_num < input {
        let new_size = current_ring.size + 8;
        let new_biggest = current_ring.biggest_num + new_size;
        let new_no = current_ring.no + 1;

        current_ring = Ring::new(new_no, new_size, new_biggest);
    }

    let position_in_ring = current_ring.biggest_num - input;
    let ring_edge_len = (current_ring.size / 4) + 1;
    let mid = ring_edge_len / 2 + 1;
    let pos_on_edge = position_in_ring % ring_edge_len;
    let dist_from_mid = (pos_on_edge as i32 - mid as i32).abs() as usize;

    current_ring.no + dist_from_mid
}

fn main() {
    let input = 312051;

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
