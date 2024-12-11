use std::error::Error;

use nom::{
    character::complete::{char, digit1},
    combinator::map_res,
    sequence::{delimited, separated_pair},
    IResult,
};

// Get a Real Rust Number from the input string
fn parse_num(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}
// Get a pair of Real Rust Numbers from a comma-separated string
fn parse_pair(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(parse_num, char(','), parse_num)(input)
}

// Get a tuple of Real Rust Numbers from the input string
fn parse_expr(input: &str) -> IResult<&str, (u32, u32)> {
    delimited(char('('), parse_pair, char(')'))(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input.txt");

    // Find all potential 'mul' statements
    let muls: Vec<(usize, &str)> = input.match_indices("mul").collect();

    // Accumulator
    let mut accum: u32 = 0;

    // Loop over the muls, and do our best to parse a mul(a,b) statement out of it. Ignore any failures.
    for mul in muls {
        let parsed = parse_expr(&input[mul.0 + 3..]);
        if parsed.is_ok() {
            let t = parsed.unwrap();
            accum += t.1 .0 * t.1 .1;
        }
    }
    println!("{}", accum);
    Ok(())
}
