use advent_of_code_2021::{input, utils::count_increases};
use failure::Error;

fn main() -> Result<(), Error> {
    let count = count_increases(&input::parse_input()?);
    println!("counter: {}", count);
    Ok(())
}
