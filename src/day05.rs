use regex::Regex;

pub fn run(input: &str) {
    // Named range regex pattern
    let range_pattern = Regex::new(r"(?<start>[0-9]+)-(?<end>[0-9]+)").unwrap();

    // Get the input and split it into fresh_ranges and ids_to_check
    let mut fresh_ranges: Vec<(usize, usize)> = input.lines()
        .filter_map(|s| range_pattern.captures(s))
        .map(|c| (c["start"].parse::<usize>().unwrap(), c["end"].parse::<usize>().unwrap()))
        .collect();

    let ids_to_check: Vec<usize> = input.lines()
        .filter(|s| !s.is_empty() && !s.contains('-'))
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    // Part 1 - total number of fresh IDs (ids_to_check which are inside any of the fresh_ranges)
    let mut fresh_id_count: usize = 0;

    for i in 0..ids_to_check.len() {
        let id = ids_to_check[i];

        let is_fresh = fresh_ranges.iter().any(|range| id >= range.0 && id <= range.1);

        if is_fresh {
            fresh_id_count += 1;
        }
    }

    println!("{fresh_id_count}");

    // Part 2 - total count of fresh IDs when range is expanded
    // Sort and dedup the fresh_ranges
    fresh_ranges.sort();

    let mut contiguous_ranges: Vec<(usize, usize)> = Vec::new();
    contiguous_ranges.push(fresh_ranges[0]);

    for i in 1..fresh_ranges.len() {

        let contiguous_index = contiguous_ranges.len() - 1;
        let latest_contiguous_range = &mut contiguous_ranges[contiguous_index];
        let current = fresh_ranges[i];

        // if there is overlap with the contiguous range, just expand the contiguous range
        if current.0 <= latest_contiguous_range.1 + 1 && current.1 > latest_contiguous_range.1 {
            latest_contiguous_range.1 = current.1;
        }
        // if the ranges do not overlap (remember, ranges are inclusive) add a new contiguous range
        else if current.0 > latest_contiguous_range.1 + 1 {
            contiguous_ranges.push(current);
        }
    }

    // Sum up the contiguous ranges
    let mut total_fresh_id_count: usize = 0;
    for i in 0..contiguous_ranges.len() {
        total_fresh_id_count += contiguous_ranges[i].1 - contiguous_ranges[i].0 + 1;
    }

    println!("{total_fresh_id_count}");
}