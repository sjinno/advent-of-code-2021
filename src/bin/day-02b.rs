use failure::Error;
use std::{
    io::{self, Read},
    str::FromStr,
};

#[derive(Debug)]
enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[derive(Debug, Default)]
struct Submarine {
    h: i32,
    v: i32,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cmd = s.split_ascii_whitespace();
        let (dir, x) = (cmd.next().unwrap(), cmd.next().unwrap().parse()?);
        match dir {
            "forward" => Ok(Instruction::Forward(x)),
            "down" => Ok(Instruction::Down(x)),
            "up" => Ok(Instruction::Up(x)),
            _ => panic!("No such instruction"),
        }
    }
}

fn main() -> Result<(), Error> {
    let mut buffer = String::new();
    {
        let stdin = io::stdin();
        stdin.lock().read_to_string(&mut buffer)?;
    }

    let mut sub = Submarine::default();
    let mut aim_enhancer = 0;

    for line in buffer.lines() {
        let instruction = Instruction::from_str(line)?;

        match instruction {
            Instruction::Forward(x) => {
                sub.h += x;
                aim_enhancer += x * sub.v;
            }
            Instruction::Down(x) => sub.v += x,
            Instruction::Up(x) => sub.v -= x,
        }
    }

    let aim = sub.h * aim_enhancer;

    println!("aim: {}", aim);

    Ok(())
}
