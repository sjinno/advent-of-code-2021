use failure::Error;
use std::collections::HashMap;
use std::io::{self, Read};

fn main() -> Result<(), Error> {
    let mut buf = String::new();
    {
        let stdin = io::stdin();
        stdin.lock().read_to_string(&mut buf)?;
    }

    let mut iter = buf.lines();

    let mut input = iter.next().unwrap().to_string();
    println!("input: {}", input);

    // let mut hm = HashMap::new::<str, char>();
    let map: HashMap<&str, &str> = iter
        .filter(|line| *line != "")
        .map(|line| {
            let mut split = line.split("->");
            let s1 = split.next().unwrap().trim();
            let s2 = split.next().unwrap().trim();
            (s1, s2)
        })
        .collect();

    // println!("map: {:?}", map);

    input
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .for_each(|w| println!("w: {}", w.collect::<String>()));
    // println!("input[..]: {:?}", &input[..]);

    // for c in input.chars() {
    //     println!("c: {}", c);
    // }

    Ok(())
}
