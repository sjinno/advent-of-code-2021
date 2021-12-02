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
    let mut depth_multiplied = 0;

    for line in buffer.lines() {
        let mut split = line.split(' ');

        match (
            split.next().unwrap(),
            split.next().unwrap().parse::<isize>()?,
        ) {
            ("forward", x) => {
                h_pos += x;
                depth_multiplied += x * depth;
            }
            ("down", x) => depth += x,
            ("up", x) => depth -= x,
            _ => {}
        }
    }

    let aim = h_pos * depth_multiplied;

    println!("aim: {}", aim);

    Ok(())
}
