use failure::Error;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Line {
    p0: Point,
    p1: Point,
}

fn main() -> Result<(), Error> {
    let mut buf = String::new();
    {
        let stdin = io::stdin();
        stdin.lock().read_to_string(&mut buf)?;
    }

    let lines = buf
        .lines()
        .map(|line| {
            let split = line
                .split("->")
                .map(|p| {
                    let mut point = p.trim().split(',');
                    Point {
                        x: point.next().unwrap().parse::<i32>().unwrap(),
                        y: point.next().unwrap().parse::<i32>().unwrap(),
                    }
                })
                .collect::<Vec<Point>>();

            Line {
                p0: split[0].clone(),
                p1: split[1].clone(),
            }
        })
        .collect::<Vec<Line>>();

    let mut hm = HashMap::<Point, usize>::new();

    for line in lines {
        let (p0, p1) = (line.p0, line.p1);

        let coord0 = (p0.x, p0.y);
        let coord1 = (p1.x, p1.y);

        match (coord0, coord1) {
            ((x0, y0), (x1, y1)) if x0 == x1 && y0 == y1 => {
                let (s, e) = if x0 < y0 { (x0, y0) } else { (y0, x0) };
                for i in s..=e {
                    let counter = hm.entry(Point { x: i, y: i }).or_insert(0);
                    *counter += 1;
                }
            }
            ((x0, y0), (x1, y1)) if (x0 - x1).abs() == (y0 - y1).abs() => {
                let new_y0;
                let new_y1;
                let (s, e) = if x0 < x1 {
                    new_y0 = y0;
                    new_y1 = y1;
                    (x0, x1)
                } else {
                    new_y0 = y1;
                    new_y1 = y0;
                    (x1, x0)
                };
                let mut a = 0;
                for i in s..=e {
                    let counter = hm
                        .entry(Point {
                            x: i,
                            y: new_y0 + a,
                        })
                        .or_insert(0);
                    *counter += 1;
                    if new_y0 < new_y1 {
                        a += 1
                    } else {
                        a -= 1;
                    }
                }
            }
            ((x0, y0), (x1, y1)) if x0 == x1 => {
                let (s, e) = if y0 < y1 { (y0, y1) } else { (y1, y0) };
                for i in s..=e {
                    let counter = hm.entry(Point { x: x0, y: i }).or_insert(0);
                    *counter += 1;
                }
            }
            ((x0, y0), (x1, y1)) if y0 == y1 => {
                let (s, e) = if x0 < x1 { (x0, x1) } else { (x1, x0) };
                for i in s..=e {
                    let counter = hm.entry(Point { x: i, y: y0 }).or_insert(0);
                    *counter += 1;
                }
            }
            _ => {}
        }
    }

    let mut count = 0;

    for (_key, val) in &hm {
        if val > &1 {
            count += 1;
        }
    }

    println!("count: {}", count);

    Ok(())
}
