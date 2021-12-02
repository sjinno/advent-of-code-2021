use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
fn solve_part1(depths: &[i32]) -> usize {
    depths.windows(2).filter(|win| win[0] < win[1]).count()
}

#[aoc(day1, part2)]
fn solve_part2(depths: &[i32]) -> usize {
    solve_part1(
        &depths
            .windows(3)
            .map(|w| w.iter().sum())
            .collect::<Vec<_>>(),
    )
}
