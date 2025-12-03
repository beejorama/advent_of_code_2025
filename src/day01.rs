pub fn run(input: &str) {

    // Collect input into a Vec
    let lines: Vec<&str> = input.lines().collect();

    // Dial starts at 50
    let mut dial: isize = 50;

    // Part 1
    // How many times does the dial land on 0?
    let mut zero_times: isize = 0;

    // Part 2
    // How many times does the dial move onto 0 (even if it doesn't stop there)?
    let mut crossover_times: isize = 0;

    for i in 0..lines.len() {
        // Parse the text line into an isize
        let direction: isize = if lines[i].chars().nth(0).unwrap() == 'R' { 1 } else { -1 };
        let magnitude: isize = lines[i][1..].parse::<isize>().unwrap();

        // Store previous values to help debug
        let pre_dial = dial;

        // Calculate how many full rotations we do
        let diff = direction * magnitude;
        crossover_times += (diff / 100).abs();

        // Update dial with the remainder
        let remainder = diff % 100;
        dial += remainder;

        // Simplify
        if dial > 99 {
            dial -= 100;

            if dial != 0 {
                crossover_times += 1;
            }
        }

        if dial < 0 {
            dial += 100;

            // Have to ensure we aren't already on 0 because we don't want to double count
            if pre_dial != 0 && dial != 0  {
                crossover_times += 1;
            }
        }

        if dial == 0 {
            zero_times += 1;
            crossover_times += 1;
        }
    }

    println!("{zero_times}");
    println!("{crossover_times}");
}