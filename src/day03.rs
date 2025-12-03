// This is a kind of two-pointers thing
// The start index moves based on the index of the digit before it
// The end index moves based on the index of the digit we're working out

pub fn run(input: &str) {
    // Get lines from input (each is a battery bank)
    let banks: Vec<&str> = input.lines().collect();
    let mut total_joltage: usize = 0;

    // let digit_count = 2; // part 1
    let digit_count = 12; // part 2

    // Iterate through the banks
    for i in 0..banks.len() {

        let bank_ints: Vec<usize> = banks[i].split("").filter_map(|b| b.parse().ok()).collect();

        // stores the battery values
        let mut digits: Vec<usize> = Vec::with_capacity(digit_count);

        // for debugging
        let bank = banks[i];
        println!("{bank}");

        let mut start_index: usize = 0;

        // Work out each digit
        for j in 0..digit_count {
            // confirm this is correct
            let end_index: usize = bank_ints.len() - digit_count + j + 1;

            let mut highest_value: usize = 0;
            let mut next_start_index: usize = start_index;

            // loop through the subset
            for k in start_index..end_index {
                if bank_ints[k] > highest_value {
                    highest_value = bank_ints[k];
                    next_start_index = k + 1;

                    if highest_value == 9 {
                        break;
                    }
                }
            }
            digits.push(highest_value);
            start_index = next_start_index;
        }

        // Add the joltage for the bank to total_joltage
        let mut multiplier: usize = 1;
        let mut bank_value: usize = 0;
        while let Some(digit) = digits.pop() {
            bank_value += digit * multiplier;
            multiplier *= 10;
        }

        println!("{bank_value}");
        total_joltage += bank_value;
    }

    println!("{total_joltage}");
}