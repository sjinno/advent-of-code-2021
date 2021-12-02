use failure::Error;
use std::io::{self, Read};

fn main() -> Result<(), Error> {
    let mut buffer = String::new();
    {
        let stdin = io::stdin();
        stdin.lock().read_to_string(&mut buffer)?;
    }

    let mut h_pos = 0;
    let mut depth = 0;

    for line in buffer.lines() {
        let mut split = line.split(' ');
        let (instruction, x) = (
            split.next().unwrap(),
            split.next().unwrap().parse::<isize>()?,
        );

        match instruction {
            "forward" => h_pos += x,
            "down" => depth += x,
            "up" => depth -= x,
            _ => {}
        }
    }

    let aim = h_pos * depth;

    println!("aim: {}", aim);

    Ok(())
}
