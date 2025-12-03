use regex::Regex;

pub fn run(input: &str) {

    // Collect input into a Vec of ranges
    let str_ranges: Vec<&str> = input.split(',').collect();
    let pattern = Regex::new(r"(?<start>[0-9]+)-(?<end>[0-9]+)").unwrap();

    // Vec for invalid IDs
    let mut invalid_ids_total_1: usize = 0;
    let mut invalid_ids_total_2: usize = 0;

    // Iterate over string ranges
    for i in 0..str_ranges.len() {
        let Some(caps) = pattern.captures(str_ranges[i]) else { return };
        let start_id: usize = caps["start"].parse::<usize>().unwrap();
        let end_id: usize = caps["end"].parse::<usize>().unwrap();

        println!("Range {start_id}-{end_id}");

        // Iterate over each id
        for j in start_id..end_id + 1 {
            // Convert to a string slice
            let j_string = j.to_string();
            let j_string_len = j_string.len();

            // Part 1 - the number's first half must equal the number's second half
            // IDs with an odd number of digits can never be invalid
            // if j_string_len % 2 != 0 {
            //     continue;
            // }

            // // Split the string in half and compare
            // let first_half = j_string[0..j_string_len/2].to_string();
            // let second_half = j_string[j_string_len/2..].to_string();

            // if first_half == second_half {
            //     println!("MATCH: {first_half}{second_half}");
            //     invalid_ids_total_1 += j;
            // }

            // Part 2 - the ID is invalid if it is a pattern repeated at least twice
            let mut sub_len = j_string_len / 2;

            while sub_len >= 1 {

                // If the string isn't divisible by sub_len, continue, because it will always be a valid ID
                if j_string_len % sub_len == 0 {

                    // Get substring
                    let substring = j_string[0..sub_len].to_string();
                    let comparison_string = substring.repeat(j_string_len / sub_len);

                    if comparison_string == j_string {
                        // println!("MATCH: {j_string} {substring}");

                        // if sub_len == j_string_len / 2 && j_string_len % 2 == 0 {
                        //     invalid_ids_total_1 += j;
                        // }

                        invalid_ids_total_2 += j;
                        break;
                    }
                }

                sub_len -= 1;
            }
        }        
    }

    println!("{invalid_ids_total_1}");
    println!("{invalid_ids_total_2}");

    // Numbers with an odd number of digits are never invalid
}