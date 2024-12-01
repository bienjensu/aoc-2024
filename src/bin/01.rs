#![feature(test)]

use itertools::Itertools;
use std::{collections::HashMap, simd::Simd};

extern crate test;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let parsed = input.split_whitespace().map(|e| e.parse::<u32>().unwrap());

    let one = parsed.clone().step_by(2).sorted();
    let two = parsed.skip(1).step_by(2).sorted();

    Some(one.zip(two).map(|(a, b)| a.abs_diff(b)).sum::<u32>())
}

pub fn part_two_orig(input: &str) -> Option<u32> {
    let parsed = input.split_whitespace().map(|e| e.parse::<u32>().unwrap());

    let one = parsed.clone().step_by(2);
    let two = parsed.skip(1).step_by(2);

    let mut num_to_freq: HashMap<u32, u32> = HashMap::new();

    for n in two {
        match num_to_freq.get_mut(&n) {
            Some(n) => *n += 1,
            None => {
                num_to_freq.insert(n, 1);
            }
        }
    }

    Some(
        one.into_iter()
            .map(|n| match num_to_freq.get(&n) {
                Some(m) => n * m,
                None => 0,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let parsed = input
        .split_whitespace()
        .map(|e| e.parse::<usize>().unwrap());

    let one = parsed.clone().step_by(2);
    let two = parsed.skip(1).step_by(2);

    let mut num_to_freq: [usize; 99999] = [0; 99999];

    for n in two {
        num_to_freq[n] += 1;
    }

    Some(one.map(|n| n * num_to_freq[n]).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = &advent_of_code::template::read_file("inputs", DAY);

        b.iter(|| part_one(input));
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = &advent_of_code::template::read_file("inputs", DAY);

        b.iter(|| part_two_orig(input));
    }

    #[bench]
    fn bench_part_two_no_hash(b: &mut Bencher) {
        let input = &advent_of_code::template::read_file("inputs", DAY);

        b.iter(|| part_two(input));
    }
}
