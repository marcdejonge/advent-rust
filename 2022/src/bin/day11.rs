#![feature(test)]
#![allow(clippy::ptr_arg)]

use advent_lib::iter_utils::IteratorUtils;
use advent_lib::*;
use nom_parse_macros::parse_from;
use std::cell::RefCell;

fn calculate_part1(monkeys: &[Monkey]) -> u64 { calculate(monkeys, 20, 3) }

fn calculate_part2(monkeys: &[Monkey]) -> u64 { calculate(monkeys, 10000, 1) }

#[parse_from]
#[derive(Debug, Clone, Copy)]
enum Operation {
    #[format("+")]
    Add,
    #[format("*")]
    Multiply,
}

#[derive(Debug, Clone)]
#[parse_from((
    delimited("Monkey ", u32, (":", line_ending)),
    delimited("  Starting items: ", separated_list1(", ", u64), line_ending),
    preceded("  Operation: new = old ", {}),
    delimited(space1, alt((u64, map("old", |_| 0))), line_ending),
    delimited("  Test: divisible by ", u64, line_ending),
    delimited("    If true: throw to monkey ", {}, line_ending),
    delimited("    If false: throw to monkey ", {}, opt(line_ending)),
))]
struct Monkey {
    _index: u32,
    items: Vec<u64>,
    operation_type: Operation,
    amount: u64,
    test_divisible_by: u64,
    true_monkey: u64,
    false_monkey: u64,
    #[derived(0)]
    inspected_items: u64,
}

fn execute_round(monkeys: &Vec<RefCell<Monkey>>, param: u64, reduce: fn(u64, u64) -> u64) {
    for monkey_cell in monkeys {
        let mut monkey = monkey_cell.borrow_mut();

        let amount = monkey.amount;
        match monkey.operation_type {
            Operation::Add => {
                for old in monkey.items.iter_mut() {
                    let add = if amount == 0 { *old } else { amount };
                    *old = reduce(*old + add, param);
                }
            }
            Operation::Multiply => {
                for old in monkey.items.iter_mut() {
                    let add = if amount == 0 { *old } else { amount };
                    *old = reduce(*old * add, param);
                }
            }
        }

        if monkey.true_monkey == monkey.false_monkey {
            let mut send_monkey = monkeys
                .get(monkey.true_monkey as usize)
                .expect("Can't find reference monkey")
                .borrow_mut();
            for &item in monkey.items.iter() {
                send_monkey.items.push(item)
            }
        } else {
            let mut true_monkey = monkeys
                .get(monkey.true_monkey as usize)
                .expect("Can't find reference true monkey")
                .borrow_mut();
            let mut false_monkey = monkeys
                .get(monkey.false_monkey as usize)
                .expect("Can't find reference false monkey")
                .borrow_mut();
            for &item in monkey.items.iter() {
                if (item % monkey.test_divisible_by) == 0 {
                    true_monkey.items.push(item)
                } else {
                    false_monkey.items.push(item)
                };
            }
        }

        monkey.inspected_items += monkey.items.len() as u64;
        monkey.items.clear();
    }
}

fn calculate_mod(monkeys: &[Monkey]) -> u64 {
    monkeys.iter().fold(1u64, |acc, monkey| acc * monkey.test_divisible_by)
}

fn calculate(monkeys: &[Monkey], rounds: u32, div: u64) -> u64 {
    let calc_monkeys = monkeys.iter().map(|m| RefCell::new(m.clone())).collect::<Vec<_>>();

    if div > 1 {
        for _ in 0..rounds {
            execute_round(&calc_monkeys, div, |x, div| x / div);
        }
    } else {
        let modulus = calculate_mod(monkeys);
        for monkey in &calc_monkeys {
            let mut monkey = monkey.borrow_mut();
            for item in monkey.items.iter_mut() {
                *item %= modulus;
            }
        }

        for _ in 0..rounds {
            execute_round(&calc_monkeys, modulus, |x, modulus| x % modulus);
        }
    }

    let [first, second] = calc_monkeys.iter().map(|m| m.borrow().inspected_items).max_n();
    first * second
}

day_main!(Vec<Monkey>);
day_test!( 11, example => 10605, 2713310158 );
day_test!( 11 => 108240, 25712998901 );
