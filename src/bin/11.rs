#![feature(iter_array_chunks)]

use std::{
    io::{self, Read},
};
use indicatif::ProgressIterator;

use itertools::Itertools;

struct Monkey {
    items: Vec<i64>,
    operation: MonkeyOperation,
    test_for: (i64, usize, usize),
    total_inspections: usize,
}

enum MonkeyOperation {
    Multiply(MonkeyValue),
    Add(MonkeyValue),
}

enum MonkeyValue {
    Identity,
    Of(i64),
}

impl MonkeyValue {
    fn get(self: &Self, y: i64) -> i64 {
        match self {
            Self::Identity => y,
            Self::Of(x) => *x,
        }
    }
}

impl MonkeyOperation {
    fn apply(self: &Self, y: i64) -> i64 {
        match self {
            Self::Multiply(x) => x.get(y) * y,
            Self::Add(x) => x.get(y) + y,
        }
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut monkeys: Vec<Monkey> = input
        .split("\n")
        .map(|s| s.trim())
        .filter(|l| l.len() > 0)
        .array_chunks::<6>()
        .map(|chunk| {
            let starting_items = chunk[1]
                .trim_start_matches("Starting items: ")
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect();
            println!("{}", chunk[2]);
            let (operation_type, operation_value_str) = chunk[2]
                .trim_start_matches("Operation: new = old ")
                .split_whitespace()
                .collect_tuple()
                .unwrap();
            let operation_value = match operation_value_str {
                "old" => MonkeyValue::Identity,
                _ => MonkeyValue::Of(operation_value_str.parse().unwrap()),
            };
            let operation = match operation_type {
                "+" => MonkeyOperation::Add(operation_value),
                "*" => MonkeyOperation::Multiply(operation_value),
                _ => panic!("Unknown op {}", operation_type),
            };
            let (test_for_divisor, test_case_if, test_case_else) = (
                chunk[3]
                    .trim_start_matches("Test: divisible by ")
                    .parse()
                    .unwrap(),
                chunk[4]
                    .trim_start_matches("If true: throw to monkey ")
                    .parse()
                    .unwrap(),
                chunk[5]
                    .trim_start_matches("If false: throw to monkey ")
                    .parse()
                    .unwrap(),
            );

            Monkey {
                items: starting_items,
                operation: operation,
                test_for: (test_for_divisor, test_case_if, test_case_else),
                total_inspections: 0,
            }
        })
        .collect();
    
    let gcd = monkeys.iter().fold(1, |b, m| b * m.test_for.0);

    for _ in (0..10000).progress() {
        let mut deposits: Vec<(usize, i64, bool)> = Vec::new();
        for (i, m) in monkeys.iter_mut().enumerate() {
            deposits
                .iter_mut()
                .filter(|(next, _, _)| *next == i)
                .for_each(|(_, worry, consumed)| {
                    *consumed = true;
                    m.items.push(*worry);
                });
            for item in m.items.iter() {
                let worry = m.operation.apply(*item);
                let next = if worry % m.test_for.0 == 0 {
                    m.test_for.1
                } else {
                    m.test_for.2
                };
                deposits.push((next, worry%gcd, false));
            }
            // wipe out items
            m.total_inspections += m.items.len();
            m.items.clear();
        }
        deposits
            .iter()
            .filter(|(_, _, consumed)| !consumed)
            .for_each(|(next, worry, _)| {
                monkeys[*next]
                    .items
                    .push(*worry)
            });
        deposits.clear();
    }

    let top_monkeys: Vec<usize> = monkeys
        .iter()
        .map(|m| m.total_inspections)
        .sorted()
        .rev()
        .collect();
    println!(
        "{}",
        top_monkeys.iter().map(|s| format!("{}", s)).join(", ")
    );
    println!("{}", top_monkeys[0..2].iter().fold(1, |x, y| x * y));

    Ok(())
}
