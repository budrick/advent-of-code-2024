use std::io::{self, BufRead};

fn main() {
    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();
    let stdin = io::stdin();
    let mut stdin_h = stdin.lock();
    let mut eof = false;
    let mut line = String::new();

    while !eof {
        match stdin_h.read_line(&mut line) {
            Ok(0) => {
                eof = true;
            }
            Ok(_) => {
                line.pop();
                let bits: Vec<&str> = line.splitn(2, "   ").collect();
                l1.push(bits[0].parse::<i32>().unwrap());
                l2.push(bits[1].parse::<i32>().unwrap());
                line.clear();
            }
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        }
    }

    assert!(
        l1.len() == l2.len(),
        "Length error: {} != {}",
        l1.len(),
        l2.len()
    );

    l1.sort();
    l2.sort();

    let mut dist: u32 = 0;
    let mut sim: u32 = 0;

    for i in 0..l1.len() {
        let diff = (l1[i] - l2[i]).abs() as u32;
        dist = dist + diff;

        let count = l2.iter().filter(|n| n == &&l1[i]).count() as u32;
        sim = sim + (count * l1[i] as u32);
    }
    println!("Count: {}", dist);
    println!("Similarity: {}", sim);
}
