use failure::Error;
use std::io::{self, Read};

fn main() -> Result<(), Error> {
    let mut buffer = String::new();
    {
        let stdin = io::stdin();
        stdin.lock().read_to_string(&mut buffer)?;
    }

    let report: Vec<Vec<u32>> = buffer
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut gamma = vec![0; report[0].len()];

    for row in &report {
        for col in 0..row.len() {
            gamma[col] += row[col];
        }
    }

    let mid = report.len() as u32 / 2;

    gamma = gamma.iter().map(|g| if *g > mid { 1 } else { 0 }).collect();
    let gamma_rate = calculate_rate(&gamma[..]);

    let epsilon: Vec<u32> = gamma.into_iter().map(|b| (b == 0) as u32).collect();
    let epsilon_rate = calculate_rate(&epsilon[..]);

    println!("var: {}", gamma_rate * epsilon_rate);

    Ok(())
}

fn calculate_rate(report: &[u32]) -> i32 {
    report
        .iter()
        .enumerate()
        .map(|(i, b)| {
            if *b == 1 {
                2_i32.pow(report.len() as u32 - 1 - i as u32)
            } else {
                0
            }
        })
        .sum()
}
