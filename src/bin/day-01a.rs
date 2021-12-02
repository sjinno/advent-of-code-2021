use failure::Error;
use std::io::Read;

fn main() -> Result<(), Error> {
    let mut buffer = String::new();
    {
        let stdin = std::io::stdin();
        stdin.lock().read_to_string(&mut buffer)?;
    }

    let mut lines = buffer.lines();
    let mut prev: u32 = lines.by_ref().take(1).next().unwrap().parse()?;
    let mut counter = 0;

    for line in lines {
        if let Ok(curr) = line.parse() {
            if curr > prev {
                counter += 1;
            }
            prev = curr;
        }
    }

    println!("counter: {}", counter);

    Ok(())
}
