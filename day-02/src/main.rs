#[derive(Debug)]
enum Direction {
    Inc,
    Dec,
}

fn check_safe(levels: &Vec<i32>) -> bool {
    let mut safe = true;
    let direction = if levels[0] < levels[1] {
        Direction::Inc
    } else {
        Direction::Dec
    };

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

    safe
}

fn check_safe_with_removals(levels: &Vec<i32>) -> bool {
    for i in 0..levels.len() {
        let mut mlevels = levels.clone();
        mlevels.remove(i);
        if check_safe(&mlevels) {
            return true;
        }
    }
    false
}

fn main() {
    let input = include_str!("../input.txt");
    let safelist = input.lines().map(|line| {
        let levels: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|l| l.parse::<i32>().unwrap())
            .collect();

        let mut safe = check_safe(&levels);

        if !safe {
            safe = check_safe_with_removals(&levels);
        }

        if safe {
            Some(levels)
        } else {
            None
        }
    });

    let fulllist: Vec<Option<Vec<i32>>> = safelist.collect();

    println!(
        "{:?}",
        fulllist
            .iter()
            .filter(|i| { i.is_some() })
            .collect::<Vec<&Option<Vec<i32>>>>()
            .len()
    );
}
