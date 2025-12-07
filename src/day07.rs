use std::collections::HashMap;

pub fn run(input: &str) {
    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let mut splits: usize = 0; // Part 1

    let mut p_beams: HashMap<usize, usize> = HashMap::new(); // (Index, Timelines)
    p_beams.insert(input.find('S').unwrap(), 1);

    for i in 1..lines.len() {
        // Split p_beams into 2 Vecs for interrupted and uninterrupted beams
        let (interrupted_beams, mut uninterrupted_beams): (Vec<(usize, usize)>, Vec<(usize, usize)>) = p_beams
            .iter()
            .map(|(&k, &v)| (k, v))
            .partition(|(k, _v)| lines[i][*k] == '^');

        // Transform the interrupted beams into the new split beams
        let mut new_beams: Vec<(usize, usize)> = interrupted_beams
            .iter()
            .map(|&ind| [(ind.0 - 1, ind.1), (ind.0 + 1, ind.1)])
            .flatten()
            .collect();

        // Add the uninterrupted beams and then aggregate duplicates
        new_beams.append(&mut uninterrupted_beams);
        new_beams.sort();
        new_beams = new_beams
            .chunk_by(|&(ka, _va), &(kb, _vb)| ka == kb)
            .map(|c| (c[0].0, c.iter().map(|d| d.1).sum::<usize>()))
            .collect();

        splits += interrupted_beams.len(); // Part 1

        p_beams = HashMap::from_iter(new_beams);
    }

    let timelines: usize = p_beams.values().sum(); // Part 2

    println!("{splits}");
    println!("{timelines}");
}