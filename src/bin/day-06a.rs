use failure::Error;
use std::io::{self, Read};

fn main() -> Result<(), Error> {
    let mut buf = String::new();
    {
        let stdin = io::stdin();
        stdin.lock().read_to_string(&mut buf)?;
    }

    let mut lanternfish: Vec<usize> = buf
        .split(',')
        .map(|x| x.parse())
        .collect::<Result<_, _>>()?;

    for _ in 0..80 {
        for i in 0..lanternfish.len() {
            if lanternfish[i] == 0 {
                lanternfish[i] = 6;
                lanternfish.push(8);
            } else {
                lanternfish[i] -= 1;
            }
        }
    }

    println!("number: {:?}", lanternfish.len());

    Ok(())
}
