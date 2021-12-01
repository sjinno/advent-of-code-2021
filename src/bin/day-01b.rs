use failure::Error;
use std::io::{self, Read};

fn main() -> Result<(), Error> {
    let mut buffer = String::new();
    {
        let stdin = io::stdin();
        stdin.lock().read_to_string(&mut buffer)?;
    }

    // let data: Vec<u32> = buffer.lines().map(|elt| elt.parse().unwrap()).collect();

    // let mut prev = data[0] + data[1] + data[2];
    // let mut counter = 0;

    // for i in 1..data.len() - 2 {
    //     let curr = data[i] + data[i + 1] + data[i + 2];
    //     if curr > prev {
    //         counter += 1;
    //     }
    //     prev = curr;
    // }

    // eprintln!("{}", counter);

    // With windows
    let data = buffer
        .lines()
        .map(|elt| elt.parse().unwrap())
        .collect::<Vec<_>>();

    let mut prev = data[0..3].iter().sum::<u32>();

    let count = data.windows(4).fold(0, |acc, win| {
        let curr = win[1..4].iter().sum::<u32>();
        if curr > prev {
            prev = curr;
            acc + 1
        } else {
            prev = curr;
            acc
        }
    });

    eprintln!("{}", count);

    Ok(())
}
