use advent_of_code_2021::input;
use failure::Error;

fn main() -> Result<(), Error> {
    let data = input::parse_input()?;
    let count = data
        .windows(4)
        .filter(|win| win[0..3].iter().sum::<u32>() < win[1..4].iter().sum::<u32>())
        .count();
    println!("count: {}", count);
    Ok(())
}
