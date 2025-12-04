// I got a lot of help with this answer, as I was going into the reeds with a HashMap approach rather than taking the more direct and obvious route of iterating through it all.
// Another thing which was new to me was manipulating the input to be in a more useful state.

pub fn run(input: &str) {
    // Get input lines - translate them into something more useful than a str
    let mut grid: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.as_bytes().to_owned())
        .collect();

    let mut total_part_1: usize = 0;
    let mut total_part_2: usize = 0;

    // Part 1
    let part_1_answer = solve_grid(&grid);
    total_part_1 += part_1_answer;

    // Part 2
    loop {
        let removals = solve_grid_with_removals(&mut grid);
        total_part_2 += removals;

        if removals == 0 {
            break;
        }
    }

    println!("{total_part_1}");
    println!("{total_part_2}");
}

fn solve_grid(grid: &Vec<Vec<u8>>) -> usize  {
    let mut total: usize = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == b'@' && count_adjacent_rugs(&grid, row, col) < 4 {
                total += 1;
            }
        }
    }

    total
}

fn solve_grid_with_removals(grid: &mut Vec<Vec<u8>>) -> usize  {
    let mut total: usize = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == b'@' && count_adjacent_rugs(&grid, row, col) < 4 {
                total += 1;
                grid[row][col] = b'.';
            }
        }
    }

    total
}

fn count_adjacent_rugs(grid: &Vec<Vec<u8>>, row: usize, col: usize) -> usize {
    let row_min = row.saturating_sub(1);
    let row_max = (row + 1).min(grid.len() - 1);
    let col_min = col.saturating_sub(1);
    let col_max = (col + 1).min(grid[0].len() - 1);

    (row_min..=row_max)
        .flat_map(|r| (col_min..=col_max).map(move |c| grid[r][c] == b'@'))
        .filter(|b| *b)
        .count()
        - 1
}