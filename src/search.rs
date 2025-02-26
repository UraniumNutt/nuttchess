use std::isize;

use crate::board::*;
use crate::eval::*;
use crate::generate::*;
use crate::tables::*;
pub fn perft(board: &mut BoardState, depth: usize) {
    let tables = Tables::new();
    let top_moves = generate(board, &tables);
    let mut total_node_count = 0;
    for lower_move in top_moves {
        if let Ok(move_name) = lower_move.to_string() {
            board.make(&lower_move);
            let lower_node_count = perft_search(board, &tables, depth - 1);
            total_node_count += lower_node_count;
            board.unmake(&lower_move);

            println!("{} {}", move_name, lower_node_count);
        } else {
            panic!();
        }
    }
    println!("\n{}", total_node_count);
}

pub fn perft_search(board: &mut BoardState, tables: &Tables, depth: usize) -> usize {
    let moves = generate(board, &tables);
    if depth == 1 {
        return moves.len();
    }
    if depth == 0 {
        return 1;
    }
    let mut node_count = 0;
    for mv in moves {
        board.make(&mv);
        node_count += perft_search(board, &tables, depth - 1);
        board.unmake(&mv);
    }
    return node_count;
}

// Prototype search
pub fn negamax(board: &mut BoardState, tables: &Tables, depth: usize) -> Result<MoveRep, String> {
    let mut max = isize::MIN;
    // If all moves result in draw, none will be picked, so set the bestmove in the event that no moved is picked
    let moves = generate(board, tables);

    let mut best_move = moves[0];
    let mut alpha = isize::MIN;
    let mut beta = isize::MAX;
    for mv in &moves {
        // println!("\nStarting search for move {}", mv.to_string().unwrap());
        board.make(&mv);
        let score = negamax_child(
            board,
            tables,
            beta.saturating_neg(),
            alpha.saturating_neg(),
            moves.len(),
            depth - 1,
        )
        .saturating_neg();
        // println!(
        //     "Completed search for move {}, with a score of {}",
        //     mv.to_string().unwrap(),
        //     score
        // );
        board.unmake(&mv);
        if score > alpha {
            alpha = score;
            if alpha >= beta {
                return Ok(*mv);
            }
            best_move = *mv;
            // println!(
            //     "The move set a new max! The previous max was {}, and the new one is {}",
            //     max, score
            // );
            // max = score;
            // best_move = *mv;
        }
    }
    return Ok(best_move);
}

fn negamax_child(
    board: &mut BoardState,
    tables: &Tables,
    mut alpha: isize,
    mut beta: isize,
    number_of_moves: usize,
    depth: usize,
) -> isize {
    let moves = generate(board, tables);
    if moves.len() == 0 {
        // println!("Zeros moves generated at a depth of {}", depth);
        // print_bitboard(board.occupancy());
        match board.white_to_move {
            true => {
                let black_attack_mask = board.black_attack_mask(&tables);
                if black_attack_mask & board.white_king == 0 {
                    // println!("Returning a draw with white to move");
                    return DRAW;
                } else {
                    // println!("Returning a lose with white to move");
                    return -WIN * (depth + 1) as isize;
                }
            }
            false => {
                let white_attack_mask = board.white_attack_mask(&tables);
                if white_attack_mask & board.black_king == 0 {
                    // println!("Returning a draw with black to move");
                    return DRAW;
                } else {
                    // println!("Returning a lose with black to move");
                    return -WIN * (depth + 1) as isize;
                }
            }
        }
    }
    if depth == 0 {
        // println!("Reached a depth of zero while there are still legal moves");
        return eval(board, tables)
            + (0.1 * (moves.len() as f64 - number_of_moves as f64)) as isize;
    }
    for mv in &moves {
        // println!("Running child search on move {}", mv.to_string().unwrap());
        board.make(&mv);
        let score = negamax_child(
            board,
            tables,
            beta.saturating_neg(),
            alpha.saturating_neg(),
            moves.len(),
            depth - 1,
        )
        .saturating_neg();
        board.unmake(&mv);

        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }
    return alpha;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn depth_zero() {
        let mut board = BoardState::starting_state();
        let tables = Tables::new();

        let node_count = perft_search(&mut board, &tables, 0);
        assert_eq!(node_count, 1);
    }

    #[test]
    fn depth_1() {
        let mut board = BoardState::starting_state();
        let tables = Tables::new();

        let node_count = perft_search(&mut board, &tables, 1);
        assert_eq!(node_count, 20);
    }

    #[test]
    fn depth_2() {
        let mut board = BoardState::starting_state();
        let tables = Tables::new();

        let node_count = perft_search(&mut board, &tables, 2);
        assert_eq!(node_count, 400);
    }

    #[test]
    fn depth_3() {
        let mut board = BoardState::starting_state();
        let tables = Tables::new();

        let node_count = perft_search(&mut board, &tables, 3);
        assert_eq!(node_count, 8902);
    }

    #[test]
    fn depth_4() {
        let mut board = BoardState::starting_state();
        let tables = Tables::new();

        let node_count = perft_search(&mut board, &tables, 4);
        assert_eq!(node_count, 197281);
    }

    // This can take a moment
    // #[test]
    // fn depth_5() {
    //     let mut board = BoardState::starting_state();
    //     let tables = Tables::new();
    //     let node_count = perft_search(&mut board, &tables, 5);
    //     assert_eq!(node_count, 4865609);
    // }

    // These can take a while
    // #[test]
    // fn depth_6() {
    //     let mut board = BoardState::starting_state();
    //     let tables = Tables::new();
    //     let node_count = perft_search(&mut board, &tables, 6);
    //     assert_eq!(node_count, 119_060_324);
    // }
    //
    #[test]
    fn kiwipete_1() {
        let mut board = BoardState::state_from_string_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 1);
        assert_eq!(node_count, 48);
    }

    #[test]
    fn kiwipete_2() {
        let mut board = BoardState::state_from_string_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 2);
        assert_eq!(node_count, 2039);
    }

    #[test]
    fn kiwipete_3() {
        let mut board = BoardState::state_from_string_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 3);

        assert_eq!(node_count, 97862);
    }

    #[test]
    fn kiwipete_4() {
        let mut board = BoardState::state_from_string_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 4);

        assert_eq!(node_count, 4085603);
    }
    // This can take a while
    // #[test]
    // fn kiwipete_5() {
    //     let mut board = BoardState::state_from_string_fen(
    //         "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string(),
    //     );
    //     let tables = Tables::new();
    //     let node_count = perft_search(&mut board, &tables, 5);

    //     assert_eq!(node_count, 193690690);
    // }

    #[test]
    fn pos3_1() {
        let mut board = BoardState::state_from_string_fen(
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 1);
        assert_eq!(node_count, 14);
    }

    #[test]
    fn pos3_2() {
        let mut board = BoardState::state_from_string_fen(
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 2);
        assert_eq!(node_count, 191);
    }

    #[test]
    fn pos3_3() {
        let mut board = BoardState::state_from_string_fen(
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 3);
        assert_eq!(node_count, 2812);
    }

    #[test]
    fn pos3_4() {
        let mut board = BoardState::state_from_string_fen(
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 4);
        assert_eq!(node_count, 43238);
    }

    // #[test]
    // fn pos3_5() {
    //     let mut board = BoardState::state_from_string_fen(
    //         "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1".to_string(),
    //     );
    //     let tables = Tables::new();
    //     let node_count = perft_search(&mut board, &tables, 5);
    //     assert_eq!(node_count, 674624);
    // }

    // #[test]
    // fn pos3_6() {
    //     let mut board = BoardState::state_from_string_fen(
    //         "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1".to_string(),
    //     );
    //     let tables = Tables::new();
    //     let node_count = perft_search(&mut board, &tables, 6);
    //     assert_eq!(node_count, 11030083);
    // }

    #[test]
    fn pos4_1() {
        let mut board = BoardState::state_from_string_fen(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 1);
        assert_eq!(node_count, 6);
    }

    #[test]
    fn pos4_2() {
        let mut board = BoardState::state_from_string_fen(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 2);
        assert_eq!(node_count, 264);
    }

    #[test]
    fn pos4_3() {
        let mut board = BoardState::state_from_string_fen(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 3);
        assert_eq!(node_count, 9467);
    }

    #[test]
    fn pos4_4() {
        let mut board = BoardState::state_from_string_fen(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 4);
        assert_eq!(node_count, 422333);
    }

    // #[test]
    // fn pos4_5() {
    //     let mut board = BoardState::state_from_string_fen(
    //         "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string(),
    //     );
    //     let tables = Tables::new();
    //     let node_count = perft_search(&mut board, &tables, 5);
    //     assert_eq!(node_count, 15833292);
    // }

    // #[test]
    // fn pos4_6() {
    //     let mut board = BoardState::state_from_string_fen(
    //         "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string(),
    //     );
    //     let tables = Tables::new();
    //     let node_count = perft_search(&mut board, &tables, 6);
    //     assert_eq!(node_count, 706045033);
    // }

    #[test]
    fn pos5_1() {
        let mut board = BoardState::state_from_string_fen(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 1);
        assert_eq!(node_count, 44);
    }

    #[test]
    fn pos5_2() {
        let mut board = BoardState::state_from_string_fen(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 2);
        assert_eq!(node_count, 1486);
    }

    #[test]
    fn pos5_3() {
        let mut board = BoardState::state_from_string_fen(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 3);
        assert_eq!(node_count, 62379);
    }

    // #[test]
    // fn pos5_4() {
    //     let mut board = BoardState::state_from_string_fen(
    //         "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ".to_string(),
    //     );
    //     let tables = Tables::new();
    //     let node_count = perft_search(&mut board, &tables, 4);
    //     assert_eq!(node_count, 2103487);
    // }

    // #[test]
    // fn pos5_5() {
    //     let mut board = BoardState::state_from_string_fen(
    //         "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ".to_string(),
    //     );
    //     let tables = Tables::new();
    //     let node_count = perft_search(&mut board, &tables, 5);
    //     assert_eq!(node_count, 89941194);
    // }

    #[test]
    fn pos6_1() {
        let mut board = BoardState::state_from_string_fen(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 1);
        assert_eq!(node_count, 46);
    }

    #[test]
    fn pos6_2() {
        let mut board = BoardState::state_from_string_fen(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 2);
        assert_eq!(node_count, 2079);
    }

    #[test]
    fn pos6_3() {
        let mut board = BoardState::state_from_string_fen(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 3);
        assert_eq!(node_count, 89890);
    }

    // #[test]
    // fn pos6_4() {
    //     let mut board = BoardState::state_from_string_fen(
    //         "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ".to_string(),
    //     );
    //     let tables = Tables::new();
    //     let node_count = perft_search(&mut board, &tables, 4);
    //     assert_eq!(node_count, 3894594);
    // }

    // #[test]
    // fn pos6_5() {
    //     let mut board = BoardState::state_from_string_fen(
    //         "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ".to_string(),
    //     );
    //     let tables = Tables::new();
    //     let node_count = perft_search(&mut board, &tables, 5);
    //     assert_eq!(node_count, 164075551);
    // }

    #[test]
    fn foo() {
        let mut board =
            BoardState::state_from_string_fen("5R2/7k/3R1R2/8/8/8/2K5/8 w - - 0 1".to_string());
        let tables = Tables::new();
        let score = negamax(&mut board, &tables, 3);
        panic!();
    }
}
