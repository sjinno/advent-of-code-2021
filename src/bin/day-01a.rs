use advent_of_code_2021::utils::count_increases;
use failure::Error;
use std::io::Read;

fn main() -> Result<(), Error> {
    let mut buffer = String::new();
    {
        let stdin = std::io::stdin();
        stdin.lock().read_to_string(&mut buffer)?;
    }

    let depths = buffer
        .lines()
        .map(|d| d.parse())
        .collect::<Result<Vec<_>, _>>()?;

    let count = count_increases(&depths);

    println!("counter: {}", count);

    Ok(())
}
