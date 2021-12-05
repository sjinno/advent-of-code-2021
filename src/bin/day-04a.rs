use failure::Error;
use std::io::{self, Read};

const BINGO_SIZE: u8 = 5;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Called(u32),
    Uncalled(u32),
}

fn main() -> Result<(), Error> {
    let mut buf = String::new();
    {
        let stdin = io::stdin();
        stdin.lock().read_to_string(&mut buf)?;
    }

    let mut reader = buf.lines();

    let calls = reader
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    // println!("calls: {:?}", calls);

    let mut boards = Vec::new();
    let mut new_board = Vec::new();

    reader.next(); // skip the initial empty line
    let mut counter = 0;
    while let Some(row) = reader.next() {
        if row.is_empty() {
            continue;
        }

        counter += 1;

        new_board.push(
            row.split_ascii_whitespace()
                .map(|num| Cell::Uncalled(num.parse::<u32>().unwrap()))
                .collect::<Vec<Cell>>(),
        );

        if counter == BINGO_SIZE {
            boards.push(new_board.clone());
            new_board.clear();
            counter = 0;
        }
    }

    // println!("boards: {:?}", boards);

    // let pos = boards[0].iter().nth(2).unwrap().iter().position(|elt| {
    //     if let Cell::Uncalled(x) = elt {
    //         *x == calls[0]
    //     } else {
    //         false
    //     }
    // });
    // println!("pos: {:?}", pos);

    let mut ans = 0;

    'outer: for call in calls.iter() {
        for board in boards.iter_mut() {
            for i in 0..board.len() {
                let pos = board[i].iter().position(|elt| {
                    if let Cell::Uncalled(x) = elt {
                        x == call
                    } else {
                        false
                    }
                });
                if let Some(col) = pos {
                    // println!("call: {}", call);
                    // println!("(row, col): ({}, {})", i, col);
                    // println!();
                    board[i][col] = Cell::Called(*call);

                    if is_bingo(&board[..], i, col) {
                        println!("BINGO!!");
                        // println!("board: {:?}", board);
                        for row in board {
                            for col in row {
                                if let Cell::Uncalled(x) = col {
                                    ans += *x;
                                }
                            }
                        }

                        println!("ans: {}", ans * call);

                        break 'outer;
                    }
                }
            }
        }
    }

    // println!("boards: {:#?}", boards);

    Ok(())
}

fn is_bingo(board: &[Vec<Cell>], row: usize, col: usize) -> bool {
    board[row].iter().all(|c| matches!(c, Cell::Called(_)))
        || board.iter().all(|row| matches!(row[col], Cell::Called(_)))
}
