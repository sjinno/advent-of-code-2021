use failure::Error;
use std::io::{self, Read};

fn main() -> Result<(), Error> {
    let mut buf = String::new();
    {
        let stdin = io::stdin();
        stdin.lock().read_to_string(&mut buf)?;
    }

    let crabs: Vec<usize> = buf
        .split(',')
        .map(|c| c.parse())
        .collect::<Result<_, _>>()?;

    println!("crabs: {:?}", crabs);

    Ok(())
}
