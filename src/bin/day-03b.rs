use failure::Error;
use std::io::{self, Read};

type Report = Vec<Vec<usize>>;

enum Censor {
    O2,
    Co2,
}

fn main() -> Result<(), Error> {
    let report = parse_input()?;

    // Call get_rating function
    let o2 = get_rating(&mut report.clone(), &Censor::O2);
    let co2 = get_rating(&mut report.clone(), &Censor::Co2);

    println!("rating: {}", o2 * co2);

    Ok(())
}

fn parse_input() -> Result<Report, Error> {
    // Parse input
    let mut buffer = String::new();
    {
        let stdin = io::stdin();
        stdin.lock().read_to_string(&mut buffer)?;
    }

    Ok(buffer
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect())
}

fn get_rating(report: &mut Report, censor: &Censor) -> i32 {
    find_rating(report, censor);
    convert_binary_to_decimal(report)
}

fn find_rating(report: &mut Report, censor: &Censor) {
    for c in 0..report[0].len() {
        if report.len() == 1 {
            break;
        }

        let mut sum = 0;

        for r in 0..report.len() {
            sum += report[r][c];
        }

        let target = get_target_binary(sum, report.len(), censor);

        report.retain(|r| r[c] == target as usize);
    }
}

fn get_target_binary(sum: usize, report_size: usize, censor: &Censor) -> bool {
    match *censor {
        Censor::O2 => sum > (report_size - 1) / 2,
        Censor::Co2 => !(sum > (report_size - 1) / 2),
    }
}

fn convert_binary_to_decimal(rating: &mut Report) -> i32 {
    rating[0]
        .iter()
        .enumerate()
        .map(|(idx, n)| {
            if *n == 1 {
                2_i32.pow(rating[0].len() as u32 - 1 - idx as u32)
            } else {
                0
            }
        })
        .sum()
}
