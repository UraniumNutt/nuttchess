/*
Copyright 2025 Ethan Thummel

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
associated documentation files (the "Software"), to deal in the Software without restriction,
including without limitation the rights to use, copy, modify, merge, publish, distribute,
sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial
portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT
OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
use std::time::Instant;
mod board;
mod comm;
mod eval;
mod generate;
mod search;
mod tables;
mod tt;

use board::BoardState;
use search::{id_search, negamax, perft};
use tables::Tables;
use tt::ZobKeys;

fn main() {
    let mut board = BoardState::starting_state();
    let zob_keys = ZobKeys::new();
    let mut running = true;

    let tables = Tables::new();
    while running {
        let line = comm::engine_in();
        let mut tokens = line.split(" ");
        match tokens.next().unwrap() {
            "uci" => {
                println!("id name nuttchess");
                println!("id author UraniumNutt / Ethan Thummel");
                println!("uciok");
            }
            "isready" => {
                println!("readyok");
            }
            "ucinewgame" => {}
            "position" => match tokens.next().unwrap() {
                "startpos" => {
                    board = BoardState::starting_state();
                    if tokens.next().is_some() {
                        for mv in tokens.by_ref() {
                            board.apply_string_move(mv.to_string(), &zob_keys);
                        }
                    }
                }
                "fen" => {
                    let parsed_board = BoardState::state_from_fen(&mut tokens);
                    match parsed_board {
                        Ok(b) => {
                            board = b;
                        }
                        Err(b) => {
                            println!("Error parsing fen string: {b}");
                        }
                    }
                    if tokens.next().is_some() {
                        for mv in tokens.by_ref() {
                            board.apply_string_move(mv.to_string(), &zob_keys);
                        }
                    }
                }
                e => println!("Unexpected value {e}"),
            },
            "print" => {
                // Pretty print the board state
                board.pretty_print_board();
            }
            "go" => match tokens.next().unwrap() {
                "perft" => {
                    let depth = tokens.next();
                    match depth {
                        Some(d) => {
                            if let Ok(parsed_d) = d.parse::<u64>() {
                                perft(&mut board, parsed_d as usize, &zob_keys);
                            } else {
                                println!("Error parsing value of depth token {d}");
                            }
                        }
                        None => {
                            println!("Expected depth after token perft");
                        }
                    }
                }
                // TODO implement more / the proper searches
                "wtime" => {
                    // FIXME this is just for testing
                    let starting_time = Instant::now();
                    // Just discard these for now
                    let w_time = tokens.next().unwrap().parse::<u64>().unwrap();
                    let _ = tokens.next();
                    let b_time = tokens.next().unwrap().parse::<u64>().unwrap();
                    let _ = tokens.next();
                    let w_inc = tokens.next().unwrap_or("").parse::<u64>().unwrap_or(0);
                    let _ = tokens.next();
                    let b_inc = tokens.next().unwrap_or("").parse::<u64>().unwrap_or(0);
                    let time_to_spend = match board.white_to_move {
                        true => (w_time / 20 + w_inc / 2) as u128,
                        false => (b_time / 20 + b_inc / 2) as u128,
                    };
                    let best_move = id_search(
                        &mut board,
                        &tables,
                        &zob_keys,
                        Some(starting_time),
                        Some(time_to_spend),
                    );
                    println!("bestmove {}", best_move.to_string());
                }
                "movetime" => {
                    let starting_time = Instant::now();
                    let ms = tokens.next().unwrap().parse::<u64>().unwrap();
                    let best_move = id_search(
                        &mut board,
                        &tables,
                        &zob_keys,
                        Some(starting_time),
                        Some(ms as u128),
                    );
                    println!("bestmove {}", best_move.to_string());
                }
                "depth" => {
                    let depth = tokens.next();
                    match depth {
                        Some(d) => {
                            if let Ok(depth_number) = d.parse::<u64>() {
                                let best_move = negamax(
                                    &mut board,
                                    &tables,
                                    &zob_keys,
                                    depth_number as usize,
                                    None,
                                    None,
                                );
                                println!("bestmove {}", best_move.to_string());
                            }
                        }
                        None => println!("Expected depth token"),
                    }
                }
                _ => {}
            },
            "quit" => {
                running = false;
            }
            _ => {}
        }
    }
}
