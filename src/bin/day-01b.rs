use failure::Error;
use std::io::{self, Read};

fn main() -> Result<(), Error> {
    let mut buffer = String::new();
    {
        let stdin = io::stdin();
        stdin.lock().read_to_string(&mut buffer)?;
    }

    let data = buffer
        .trim()
        .split('\n')
        .map(|elt| elt.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut prev = data[0] + data[1] + data[2];
    let mut counter = 0;

    for i in 1..data.len() - 2 {
        let curr = data[i] + data[i + 1] + data[i + 2];
        if curr > prev {
            counter += 1;
        }
        prev = curr;
    }

    eprintln!("{}", counter);

    Ok(())
}
