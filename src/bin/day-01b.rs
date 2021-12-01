use advent_of_code_2021::{input, utils::count_increases};
use failure::Error;

fn main() -> Result<(), Error> {
    let data = input::parse_input()?;
    let count = count_increases(
        &data
            .windows(3)
            .map(|win| win.iter().sum::<u32>())
            .collect::<Vec<u32>>(),
    );
    println!("count: {}", count);
    Ok(())
}
