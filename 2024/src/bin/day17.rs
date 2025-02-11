#![feature(test)]

use advent_lib::day_main;
use advent_lib::parsing::{double_line_ending, separated_array};
use fxhash::FxHashMap;
use nom_parse_macros::parse_from;
use smallvec::SmallVec;
use std::fmt::Debug;

#[parse_from(preceded("Program: ", map(separated_list1(",", u8), SmallVec::from)))]
#[derive(Debug, Clone, Eq, PartialEq)]
struct Program(SmallVec<[u8; 16]>);

#[parse_from(preceded(delimited("Register ", alpha1, ": "), u64))]
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
struct Register(u64);

impl From<Program> for u64 {
    fn from(value: Program) -> Self { value.0.iter().fold(0, |acc, &x| acc * 10 + x as u64) }
}

impl From<&Program> for Machine {
    fn from(program: &Program) -> Self {
        Machine { registers: Default::default(), ip: 0, program: program.clone() }
    }
}

#[parse_from(separated_pair(separated_array(line_ending), double_line_ending, ()))]
struct MachineStart {
    registers: [Register; 3],
    program: Program,
}

#[derive(Clone, Debug)]
struct Machine {
    registers: [Register; 3],
    ip: usize,
    program: Program,
}

impl Machine {
    fn new(day: &MachineStart) -> Self {
        Machine { registers: day.registers, ip: 0, program: day.program.clone() }
    }

    fn combo(&self) -> u64 {
        let combo = self.literal();
        match combo {
            0..=3 => combo,
            4 => self.registers[0].0,
            5 => self.registers[1].0,
            6 => self.registers[2].0,
            _ => panic!("Invalid combo number: {}", combo),
        }
    }

    fn literal(&self) -> u64 { self.program.0[self.ip + 1] as u64 }

    fn operand(&self) -> Option<&u8> { self.program.0.get(self.ip) }

    fn a(&mut self) -> &mut u64 { &mut self.registers[0].0 }
    fn b(&mut self) -> &mut u64 { &mut self.registers[1].0 }
    fn c(&mut self) -> &mut u64 { &mut self.registers[2].0 }

    fn execute_single_output(&mut self) -> Option<u8> {
        while let Some(&opcode) = self.operand() {
            match opcode {
                0 => *self.a() >>= self.combo(),
                1 => *self.b() ^= self.literal(),
                2 => *self.b() = self.combo() % 8,
                3 => {
                    if *self.a() != 0 {
                        self.ip = self.combo() as usize;
                        continue;
                    }
                }
                4 => *self.b() ^= *self.c(),
                5 => {
                    let output = (self.combo() % 8) as u8;
                    self.ip += 2;
                    return Some(output);
                }
                6 => *self.b() = *self.a() >> self.combo(),
                7 => *self.c() = *self.a() >> self.combo(),
                _ => panic!("Invalid opcode: {}", opcode),
            }
            self.ip += 2;
        }

        None
    }

    fn execute_program(&mut self) -> Program {
        let mut output = SmallVec::new();
        while let Some(value) = self.execute_single_output() {
            output.push(value);
        }
        Program(output)
    }
}

fn calculate_part1(start: &MachineStart) -> u64 { Machine::new(start).execute_program().into() }

/*
 * All the programs I've analyzed have the same pattern. It's a loop where B and C always
 * depend on the current value of A. And every loop A gets shifted right by 3 bits.
 * To find a value that outputs the same program, I'm going to test possible input values for A
 * and see if the output matches the program. Then combine these outputs to find the final value.
 *
 * Pseudocode version of our input:
 * do {
 *   B = (A % 8) ^ 3  // 2,4, 1,3
 *   C = A >> B       // 7,5
 *   A = A >> 3       // 0,3
 *   B = B ^ 5 ^ C    // 1,5, 4,1
 *   print(B % 8)     // 5,5
 * } while a != 0     // 3,0
 */
fn calculate_part2(machine_start: &MachineStart) -> u64 {
    type ResultMap = FxHashMap<(u8, u64), Vec<u64>>;
    let mut result_map = ResultMap::default();

    // Execute with any 10-bit value for A, max 7 before the 3-bit shift might have an impact
    for start in 0..1024 {
        let mut machine = Machine::from(&machine_start.program);
        machine.registers[0].0 = start;
        if let Some(result) = machine.execute_single_output() {
            result_map.entry((result, machine.registers[0].0)).or_default().push(start);
        }
    }

    // Use a recursive function to find possible working values for register A
    fn search(program: &Program, ix: usize, a: u64, result_map: &ResultMap) -> Option<u64> {
        let output = *program.0.get(ix)?;
        for possible in result_map.get(&(output, a & 0x7F))?.iter() {
            let a = a << 3 | possible;
            if ix == 0 {
                return Some(a);
            } else if let Some(next) = search(program, ix - 1, a, result_map) {
                return Some(next);
            }
        }
        None
    }

    let a = search(
        &machine_start.program,
        machine_start.program.0.len() - 1,
        0,
        &result_map,
    )
    .expect("No solution found");

    // Verify that the found solution really works
    let mut machine = Machine::new(machine_start);
    machine.registers[0].0 = a;
    assert_eq!(machine.execute_program(), machine_start.program);

    a
}

day_main!();

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 17, example => 4635635210 );
    day_test!( 17, example2 => 5730, 117440 );
    day_test!( 17 => 167430506, 216148338630253 );
}
