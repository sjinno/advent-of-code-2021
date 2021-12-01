use advent_of_code_2021::input;
use failure::Error;

fn main() -> Result<(), Error> {
    let data = input::parse_input()?;
    let count = data.windows(2).filter(|win| win[0] < win[1]).count();
    println!("count: {}", count);
    Ok(())
}
