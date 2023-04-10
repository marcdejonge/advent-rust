use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::iter_utils::{max_n, ChunkedTrait};
use std::cell::RefCell;

struct Day {
    monkeys: Vec<Monkey>,
}

impl FromIterator<String> for Day {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Day {
            monkeys: iter
                .into_iter()
                .chunk_by("".to_owned())
                .enumerate()
                .map(|(index, lines)| parse_monkey(index, lines))
                .collect(),
        }
    }
}

impl ExecutableDay for Day {
    type Output = u64;

    fn calculate_part1(&self) -> Self::Output { self.calculate(20, 3) }

    fn calculate_part2(&self) -> Self::Output { self.calculate(10000, 1) }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation_type: char,
    amount: u64,
    test_divisible_by: u64,
    true_monkey: usize,
    false_monkey: usize,
    inspected_items: u64,
}

fn parse_monkey(index: usize, lines: Vec<String>) -> Monkey {
    if lines.len() != 6 {
        panic!("Expected 6 lines for each monkey")
    }
    if lines[0] != format!("Monkey {index}:") {
        panic!("Expected monkey in order, but starting with {}", lines[0])
    }

    let operation_line = lines[2].strip_prefix("  Operation: new = old ").expect("");

    Monkey {
        items: lines[1]
            .strip_prefix("  Starting items: ")
            .expect("Expected the starting items on the second line")
            .split(", ")
            .map(|str| str.parse::<u64>().expect("Expected only numbers"))
            .collect(),
        operation_type: operation_line.chars().next().expect("Expected some operation"),
        amount: if operation_line == "* old" {
            0
        } else {
            operation_line[2..].parse::<u64>().expect("Expected a number")
        },
        test_divisible_by: lines[3]
            .strip_prefix("  Test: divisible by ")
            .expect("Expected the divisible by test on line 4")
            .parse::<u64>()
            .expect("Expected a divisible number"),
        true_monkey: lines[4]
            .strip_prefix("    If true: throw to monkey ")
            .expect("Expected the monkey to throw at on line 5")
            .parse::<usize>()
            .expect("Expected an index to throw to"),
        false_monkey: lines[5]
            .strip_prefix("    If false: throw to monkey ")
            .expect("Expected the monkey to throw at on line 6")
            .parse::<usize>()
            .expect("Expected an index to throw to"),
        inspected_items: 0,
    }
}

fn execute_round(monkeys: &Vec<RefCell<Monkey>>, div: u64, modulus: u64) {
    for monkey_cell in monkeys {
        let mut monkey = monkey_cell.borrow_mut();
        let mut true_monkey = monkeys
            .get(monkey.true_monkey)
            .expect("Can't find reference true monkey")
            .borrow_mut();
        let mut false_monkey = monkeys
            .get(monkey.false_monkey)
            .expect("Can't find reference false monkey")
            .borrow_mut();
        for &old in monkey.items.iter() {
            let amount = if monkey.amount == 0 { old } else { monkey.amount };
            let new = (if monkey.operation_type == '+' { old + amount } else { old * amount })
                % modulus
                / div;
            if (new % monkey.test_divisible_by) == 0 {
                true_monkey.items.push(new)
            } else {
                false_monkey.items.push(new)
            };
        }

        monkey.inspected_items += monkey.items.len() as u64;
        monkey.items.clear();
    }
}

impl Day {
    fn calculate_mod(&self) -> u64 {
        self.monkeys.iter().fold(1u64, |acc, monkey| acc * monkey.test_divisible_by)
    }

    fn calculate(&self, rounds: u32, div: u64) -> u64 {
        let modulus = self.calculate_mod();
        let calc_monkeys = self.monkeys.iter().map(|m| RefCell::new(m.clone())).collect::<Vec<_>>();
        for _ in 0..rounds {
            execute_round(&calc_monkeys, div, modulus);
        }
        let [first, second] = max_n(calc_monkeys.iter().map(|m| m.borrow().inspected_items));
        first * second
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 11, example => 10605, 2713310158 );
    day_test!( 11 => 108240, 25712998901 );
}
