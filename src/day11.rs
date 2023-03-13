use crate::iter_utils::{max_n, ChunkedTrait};

crate::day!(11, Vec<Monkey>, u64 {
    parse_input(input) {
        input.lines().chunk_by("").enumerate().map(|(index, lines)| {
            parse_monkey(index, lines)
        }).collect()
    }

    calculate_part1(input) {
        calculate(input.clone(), 20, 3)
    }

    calculate_part2(input) {
        calculate(input.clone(), 10000, 1)
    }

    test example_input(include_str!("example_input/day11.txt") => 10605, 2713310158)
});

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

fn parse_monkey(index: usize, lines: Vec<&str>) -> Monkey {
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
        operation_type: operation_line
            .chars()
            .next()
            .expect("Expected some operation"),
        amount: if operation_line == "* old" {
            0
        } else {
            operation_line[2..]
                .parse::<u64>()
                .expect("Expected a number")
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

fn calculate_mod(monkeys: &Vec<Monkey>) -> u64 {
    monkeys
        .iter()
        .fold(1u64, |acc, monkey| acc * monkey.test_divisible_by)
}

fn execute_round(monkeys: &mut Vec<Monkey>, div: u64, modulus: u64) {
    for index in 0..monkeys.len() {
        let monkey = monkeys.get(index).expect("");
        let mut true_monkey_items = monkeys.get(monkey.true_monkey).expect("").items.clone();
        let mut false_monkey_items = monkeys.get(monkey.false_monkey).expect("").items.clone();
        let mut monkey = monkey.clone();
        for &old in &monkey.items {
            let amount = if monkey.amount == 0 { old } else { monkey.amount };
            let new = (if monkey.operation_type == '+' { old + amount } else { old * amount })
                % modulus
                / div;
            if (new % monkey.test_divisible_by) == 0 {
                true_monkey_items.push(new)
            } else {
                false_monkey_items.push(new)
            };
            monkey.inspected_items += 1;
        }

        monkeys[monkey.true_monkey].items = true_monkey_items;
        monkeys[monkey.false_monkey].items = false_monkey_items;
        monkey.items = Vec::new();
        monkeys[index] = monkey;
    }
}

fn calculate(mut monkeys: Vec<Monkey>, rounds: u32, div: u64) -> u64 {
    let modulus = calculate_mod(&monkeys);
    for _ in 0..rounds {
        execute_round(monkeys.as_mut(), div, modulus);
    }
    let [first, second] = max_n(monkeys.iter().map(|m| m.inspected_items));
    first * second
}
