use failure::Error;
use std::io::{self, Read};

pub fn parse_input() -> Result<Vec<u32>, Error> {
    let mut buffer = String::new();
    {
        let stdin = io::stdin();
        stdin.lock().read_to_string(&mut buffer)?;
    }
    Ok(buffer
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<u32>, _>>()?)
}
