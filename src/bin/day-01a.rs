use failure::Error;
use std::io::{self, Read};

fn main() -> Result<(), Error> {
    let mut buffer = String::new();
    {
        let stdin = io::stdin();
        stdin.lock().read_to_string(&mut buffer)?;
    }

    let mut lines = buffer.lines();
    let mut prev = lines.next().unwrap().parse::<usize>()?; // the very first number
    let mut counter = 0;

    for line in buffer.lines() {
        if let Ok(curr) = line.trim().parse::<usize>() {
            if curr > prev {
                counter += 1;
            }
            prev = curr;
        }
    }

    eprintln!("{}", counter);

    Ok(())
}
