fn search_at_pos(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    word: &Vec<char>,
) -> Option<Vec<(usize, usize)>> {
    let word_len = word.len();
    let grid_len = grid.len();
    let grid_width = grid[0].len();

    let mut results: Vec<(usize, usize)> = vec![];

    if grid[row][col] != word[0] {
        return None;
    }

    for direction in 0..8 {
        let mut cx = row as i64 + X[direction];
        let mut cy = col as i64 + Y[direction];
        let mut k = 1;

        for ki in 1..word_len {
            // Going out of our bounds? Break.
            if cx >= grid_len as i64 || cx < 0 || cy >= grid_width as i64 || cy < 0 {
                break;
            }

            // println!("{}, {}", direction, grid[cx as usize][cy as usize]);

            if grid[cx as usize][cy as usize] != word[k] {
                break;
            }

            cx += X[direction];
            cy += Y[direction];
            k = ki + 1;
        }

        if k == word_len {
            results.push((row, col));
        }
    }
    if results.len() > 0 {
        Some(results)
    } else {
        None
    }
}

fn search_for_word(grid: &Vec<Vec<char>>, word: &Vec<char>) -> Vec<(usize, usize)> {
    let mut ret: Vec<(usize, usize)> = vec![];

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let res = search_at_pos(grid, row, col, word);
            if res.is_some() {
                ret.extend(res.unwrap());
            }
        }
    }

    ret
}

const WORD: &str = "XMAS";
const X: [i64; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
const Y: [i64; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];

fn main() {
    let input: Vec<Vec<char>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let word_chars: Vec<char> = WORD.chars().collect();

    let found = search_for_word(&input, &word_chars);
    println!("Found: {}", found.len());
}
