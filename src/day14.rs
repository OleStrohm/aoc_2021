use itertools::Itertools;
use std::collections::HashMap;

pub fn day14() {
    let s = include_str!("day14.txt");

    let start = s.lines().next().unwrap().chars().collect_vec();
    let rules: HashMap<(char, char), char> = s
        .lines()
        .skip(2)
        .map(|l| {
            let (pat_left, pat_right, insert) = l
                .split(" -> ")
                .flat_map(str::chars)
                .collect_tuple()
                .unwrap();
            ((pat_left, pat_right), insert)
        })
        .collect();

    let mut memo: HashMap<((char, char), u32), HashMap<char, usize>> = HashMap::new();

    fn expand(
        pair: (char, char),
        level: u32,
        rules: &HashMap<(char, char), char>,
        memo: &mut HashMap<((char, char), u32), HashMap<char, usize>>,
    ) -> HashMap<char, usize> {
        if level == 0 {
            return HashMap::new();
        }

        let mut counts = HashMap::new();
        if let Some(&insert) = rules.get(&pair) {
            *counts.entry(insert).or_default() += 1;
            let left_pair = (pair.0, insert);
            let right_pair = (insert, pair.1);
            if !memo.contains_key(&(left_pair, level - 1)) {
                let memory = expand(left_pair, level - 1, rules, memo);
                memo.insert((left_pair, level - 1), memory);
            }

            if !memo.contains_key(&(right_pair, level - 1)) {
                let memory = expand(right_pair, level - 1, rules, memo);
                memo.insert((right_pair, level - 1), memory);
            }

            for (&ch, &count) in memo.get(&(left_pair, level - 1)).unwrap() {
                *counts.entry(ch).or_default() += count;
            }
            for (&ch, &count) in memo.get(&(right_pair, level - 1)).unwrap() {
                *counts.entry(ch).or_default() += count;
            }
        }

        counts
    }

    //let (min, max) =
    let mut counts = start.iter().copied().counts();
    for pair in start.windows(2) {
        for (ch, count) in expand((pair[0], pair[1]), 40, &rules, &mut memo) {
            *counts.entry(ch).or_default() += count;
        }
    }

    let (min, max) = counts
        .into_iter()
        .map(|(_, c)| c)
        .minmax()
        .into_option()
        .unwrap();

    println!("part 1: {}", max - min);
}
