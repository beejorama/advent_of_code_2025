/*
    Comparing my initial solutions to others online, I can see mine didn't run particularly fast.

    I didn't need to create the Calculation struct and HashMap - I could have just used the Vec containing the operators and a Vec for the numbers, using the indices.
    Other Rust solutions appear to use far fewer 'for' loops, whereas I'm still discovering the functions available to be to manipulate Vecs.

    I redid part_1 
*/

use std::collections::HashMap;

pub fn run(input: &str) {
    // Get input lines
    let (operations_lines, number_lines): (Vec<&str>, Vec<&str>) = input.lines().partition(|l| l.contains('+'));

    let part_1 = part_1(operations_lines[0], number_lines.clone());
    println!("{part_1}");

    let part_2 = part_2(operations_lines[0], number_lines);
    println!("{part_2}");
}

// Redid this part after seeing other solutions online. My previous solution used an unnecessary HashMap.
fn part_1(operations_lines: &str, number_lines: Vec<&str>) -> usize {
    let columns: Vec<char> = operations_lines
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let rows: Vec<Vec<u64>> = number_lines
        .iter()
        .map(|l| l.split_whitespace()
            .map(|n| n.parse::<u64>().unwrap()).collect())
        .collect();

    let total: u64 = columns.iter().enumerate().map(|(i, c)| {
        match c {
            '*' => rows.iter().map(|row| row[i]).product(),
            '+' => rows.iter().map(|row| row[i]).sum(),
            _ => 0 as u64
        }
    })
    .sum();

    total as usize
}

// Creates a HashMap of Calculations, but uses the index of the operator char as the HashMap key and
// processes both the operations_line and the number_lines as strings, rather than transforming into vecs.
fn part_2(operations_line: &str, number_lines: Vec<&str>) -> usize {
    let mut map: HashMap<usize, Calculation> = HashMap::with_capacity(1000);
    let mut operations_string = operations_line.to_string();

    while operations_string.len() > 0 {
        // HashMap key
        let starting_index = operations_line.len() - operations_string.len();

        // Trim the first character off to get the operator
        let op = operations_string.drain(0..1).collect::<Vec<char>>()[0];
        let mut number_count = operations_string.len();

        // Work out how many numbers in the Calculation by counting the spaces removed (between the operators)
        operations_string = operations_string.trim_start().to_string();
        number_count = 2.max(number_count - operations_string.len()); // Must be at least 2 to protect the last Calculation

        // Add the calculation to the map
        map.insert(starting_index, 
            Calculation { 
                operation: op, 
                numbers: vec![0; number_count] 
            });
    }

    // Then iterate the other lines and populate a HashMap for values
    for i in 0..number_lines.len() {
        let chars: Vec<char> = number_lines[i].chars().collect();
        let mut calc_starting_index: usize = 0;

        for j in 0..chars.len() {
            if map.contains_key(&j) {
                calc_starting_index = j;
            }

            if chars[j].is_whitespace() {
                continue;
            }

            // Otherwise add the number to the calculation
            let calc_number_index = j - calc_starting_index;

            map
                .entry(calc_starting_index)
                .and_modify(|c| {
                    if c.numbers[calc_number_index] == 0 {
                        c.numbers[calc_number_index] = chars[j].to_digit(10).unwrap() as usize
                    }
                    else {
                        c.numbers[calc_number_index] *= 10;
                        c.numbers[calc_number_index] += chars[j].to_digit(10).unwrap() as usize;
                    }
                });
        }
    }

    // Once finished, sum values.values
    let mut sum: usize = 0;
    for calc in map.values() {
        sum += calc.calculate();
    }
    sum
}

struct Calculation {
    operation: char,
    numbers: Vec<usize>
}

impl Calculation {
    fn calculate(&self) -> usize {
        let operation: Box<dyn Fn(usize, &usize) -> usize> = match self.operation {
            '*' => Box::new(|x, y| x * y),
            '+' => Box::new(|x, y| x + y),
            _ => panic!("Invalid operation")
        };

        let mut total: usize = self.numbers[0];

        for num in self.numbers.iter().skip(1) {
            total = operation(total, num);
        }

        total
    }
}