use std::env;
use std::fs;
use std::time::Instant;

fn elapsed_since(start_time: &Instant) -> String {
    let elapsed = start_time.elapsed().as_micros();
    if elapsed >= 1_000_000 {
        let elapsed = elapsed as f64 / 1_000_000.0;
        format!("{elapsed:.1}s")
    } else if elapsed >= 1000 {
        let elapsed = elapsed as f64 / 1000.0;
        format!("{elapsed:.1}ms")
    } else {
        format!("{elapsed}µs")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let days: Vec<_> = match args.len() {
        1 => (1..=12).collect(),
        _ => args.iter().skip(1).map(|d| d.parse().unwrap()).collect(),
    };
    let global_start_time = Instant::now();
    for day in &days {
        println!("Day {}:", day);
        let path = format!("./data/day{:02}.txt", day);
        let input = fs::read_to_string(&path);
        let start_time = Instant::now();
        if let Ok(input) = input {
            let input = input.trim_end();
            let day_func = match day {
                1 => advent_of_code_2025::day01::run,
                2 => advent_of_code_2025::day02::run,
                3 => advent_of_code_2025::day03::run,
                4 => advent_of_code_2025::day04::run,
                5 => advent_of_code_2025::day05::run,
                6 => advent_of_code_2025::day06::run,
                // 7 => advent_of_code_2025::day07::run,
                // 8 => advent_of_code_2025::day08::run,
                // 9 => advent_of_code_2025::day09::run,
                // 10 => advent_of_code_2025::day10::run,
                // 11 => advent_of_code_2025::day11::run,
                // 12 => advent_of_code_2025::day12::run,
                _ => unreachable!(),
            };
            day_func(input);
            println!("Time: {}", elapsed_since(&start_time));
        } else {
            println!("ERROR: no data");
        }
        println!();
    }
    if days.len() > 1 {
        println!("TOTAL TIME: {}", elapsed_since(&global_start_time));
    }
}