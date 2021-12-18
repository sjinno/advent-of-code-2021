use failure::Error;
use std::io::{self, Read};

fn main() -> Result<(), Error> {
    let mut buf = String::new();
    {
        let stdin = io::stdin();
        stdin.lock().read_to_string(&mut buf)?;
    }

    let fish = buf
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .fold([0; 9], |mut acc, t| {
            acc[t] += 1;
            acc
        });
    println!("fish: {:?}", fish);

    let count: usize = (0..256)
        .fold(fish, |[t0, t1, t2, t3, t4, t5, t6, t7, t8], _| {
            [t1, t2, t3, t4, t5, t6, t7 + t0, t8, t0]
        })
        .into_iter()
        .sum();

    println!("count: {}", count);

    Ok(())
}
