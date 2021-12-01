use advent_of_code_2021::{input, utils::count_increases};
use failure::Error;

fn main() -> Result<(), Error> {
    let data = input::parse_input()?;
    let count = count_increases(&data);
    println!("count: {}", count);
    Ok(())
}
