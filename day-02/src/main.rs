use std::collections::VecDeque;

#[derive(Debug)]
enum Direction {
    Inc,
    Dec,
}

fn main() {
    let input = include_str!("../input.txt");
    let safelist = input.lines().map(|line| {
        let levels: VecDeque<i32> = line
            .split_ascii_whitespace()
            .map(|l| l.parse::<i32>().unwrap())
            .collect();

        let direction = if levels[0] < levels[1] {
            Direction::Inc
        } else {
            Direction::Dec
        };

        let mut safe = true;

        for i in 0..levels.len() - 1 {
            let pair = (levels[i], levels[i + 1]);
            safe = match direction {
                Direction::Inc => {
                    let diff = pair.1 - pair.0;
                    pair.0 < pair.1 && diff <= 3 && diff >= 1
                }
                Direction::Dec => {
                    let diff = pair.0 - pair.1;
                    pair.1 < pair.0 && diff <= 3 && diff >= 1
                }
            };

            if !safe {
                break;
            };
        }
        if safe {
            Some(levels)
        } else {
            None
        }
    });

    println!(
        "{:?}",
        safelist
            .filter(|i| { i.is_some() })
            .collect::<Vec<Option<VecDeque<i32>>>>()
            .len()
    );
}
