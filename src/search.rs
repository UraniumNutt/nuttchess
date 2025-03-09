use std::cmp::Ordering;
use std::isize;
use std::iter::zip;
use std::time::Instant;

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
pub fn negamax(
    board: &mut BoardState,
    tables: &Tables,
    depth: usize,
    timer: Option<Instant>,
    duration: Option<u128>,
) -> Result<MoveRep, String> {
    let mut max = isize::MIN;
    // If all moves result in draw, none will be picked, so set the bestmove in the event that no moved is picked
    let mut moves = generate(board, tables);

    let mut best_move = moves[0];
    let mut alpha = isize::MIN;
    let mut beta = isize::MAX;
    let mut node_count = 0;
    for mv in &moves {
        board.make(&mv);
        let score = negamax_child(
            board,
            tables,
            beta.saturating_neg(),
            alpha.saturating_neg(),
            moves.len(),
            depth - 1,
            timer,
            duration,
            &mut node_count,
        )
        .saturating_neg();
        board.unmake(&mv);
        if score > alpha {
            alpha = score;
            if alpha >= beta {
                return Ok(*mv);
            }
            best_move = *mv;
        }
    }
    return Ok(best_move);
}

fn negamax_child(
    board: &mut BoardState,
    tables: &Tables,
    mut alpha: isize,
    mut beta: isize,
    last_number_of_moves: usize,
    depth: usize,
    timer: Option<Instant>,
    duration: Option<u128>,
    node_count: &mut usize,
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
        *node_count += 1;
        // TODO Investigate delta pruning. The fact that depth limits greater than 2 dont really improve preformance suggests
        // that this is not really a great approach
        return quiescence(board, tables, alpha, beta, 2, timer, duration, moves.len());
    }
    for mv in &moves {
        match (timer, duration) {
            (Some(t), Some(d)) => {
                if t.elapsed().as_millis() > d {
                    return DRAW;
                }
            }
            _ => {}
        }
        // println!("Running child search on move {}", mv.to_string().unwrap());
        board.make(&mv);
        let score = negamax_child(
            board,
            tables,
            beta.saturating_neg(),
            alpha.saturating_neg(),
            moves.len(),
            depth - 1,
            timer,
            duration,
            node_count,
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

/// Preformes a search using iterative deepening
pub fn id_search(
    board: &mut BoardState,
    tables: &Tables,
    depth: usize,
    timer: Option<Instant>,
    duration: Option<u128>,
    node_count: &mut usize,
) -> MoveRep {
    // Create a vector of tuples containing the possible moves, and the scores of those moves
    let mut moves = generate(board, tables);
    let mut scores = vec![0; moves.len()];
    let mut move_scores: Vec<(MoveRep, isize)> = zip(moves, scores).collect();
    let move_length = move_scores.len();
    let mut current_best = move_scores[0].0;
    let mut current_depth = 1;

    while !timer_check(timer, duration) {
        // First, generate the scores
        let mut alpha = isize::MIN;
        let mut beta = isize::MAX;

        for ms in &mut move_scores {
            if timer_check(timer, duration) {
                return current_best;
            }
            board.make(&ms.0);
            ms.1 = negamax_child(
                board,
                tables,
                alpha,
                beta,
                move_length,
                current_depth,
                timer,
                duration,
                node_count,
            );
            board.unmake(&ms.0);
        }

        // Now sort the move scores for the next iteration
        move_scores.sort_by(|a, b| a.1.cmp(&b.1));
        current_best = move_scores[0].0;
        current_depth += 1;
    }

    current_best
}

/// Preform the quiescence search
fn quiescence(
    board: &mut BoardState,
    tables: &Tables,
    mut alpha: isize,
    mut beta: isize,
    depth: usize,
    timer: Option<Instant>,
    duration: Option<u128>,
    last_number_moves: usize,
) -> isize {
    let moves = generate(board, tables);
    let number_moves = moves.len();
    let initial_eval = eval(board, tables, number_moves, last_number_moves);
    // TODO Investigate using delta pruning instead of an arbitrary depth limit
    if depth == 0 {
        return initial_eval;
    }
    let mut best_value = initial_eval;

    if initial_eval >= beta {
        return initial_eval;
    }
    if alpha < initial_eval {
        alpha = initial_eval;
    }

    for mv in &moves {
        if timer_check(timer, duration) {
            break;
        }
        // TODO Make a diffrent move generation function which only produces captures
        if mv.ending_square & board.occupancy() == 0 {
            // Skip non captures
            continue;
        }
        board.make(&mv);
        let score = quiescence(
            board,
            tables,
            beta.saturating_neg(),
            alpha.saturating_neg(),
            depth - 1,
            timer,
            duration,
            number_moves,
        )
        .saturating_neg();
        board.unmake(&mv);
        if score >= beta {
            return score;
        }
        if score > best_value {
            best_value = score;
        }
        if score > alpha {
            alpha = score;
        }
    }

    best_value
}

pub fn timer_check(timer: Option<Instant>, duration: Option<u128>) -> bool {
    match (timer, duration) {
        (Some(t), Some(d)) => {
            if t.elapsed().as_millis() > d {
                return true;
            } else {
                return false;
            }
        }
        _ => return false,
    }
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

    #[ignore = "Takes a while"]
    #[test]
    fn depth_5() {
        let mut board = BoardState::starting_state();
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 5);
        assert_eq!(node_count, 4865609);
    }

    #[ignore = "Takes a while"]
    #[test]
    fn depth_6() {
        let mut board = BoardState::starting_state();
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 6);
        assert_eq!(node_count, 119_060_324);
    }

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

    #[ignore = "Takes a while"]
    #[test]
    fn kiwipete_4() {
        let mut board = BoardState::state_from_string_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 4);

        assert_eq!(node_count, 4085603);
    }

    #[ignore = "Takes a while"]
    #[test]
    fn kiwipete_5() {
        let mut board = BoardState::state_from_string_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 5);

        assert_eq!(node_count, 193690690);
    }

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

    #[test]
    fn pos3_5() {
        let mut board = BoardState::state_from_string_fen(
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 5);
        assert_eq!(node_count, 674624);
    }

    #[ignore = "Takes a while"]
    #[test]
    fn pos3_6() {
        let mut board = BoardState::state_from_string_fen(
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 6);
        assert_eq!(node_count, 11030083);
    }

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

    #[ignore = "Takes a while"]
    #[test]
    fn pos4_5() {
        let mut board = BoardState::state_from_string_fen(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 5);
        assert_eq!(node_count, 15833292);
    }

    #[ignore = "Takes a while"]
    #[test]
    fn pos4_6() {
        let mut board = BoardState::state_from_string_fen(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 6);
        assert_eq!(node_count, 706045033);
    }

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

    #[ignore = "Takes a while"]
    #[test]
    fn pos5_4() {
        let mut board = BoardState::state_from_string_fen(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 4);
        assert_eq!(node_count, 2103487);
    }

    #[ignore = "Takes a while"]
    #[test]
    fn pos5_5() {
        let mut board = BoardState::state_from_string_fen(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 5);
        assert_eq!(node_count, 89941194);
    }

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

    #[ignore = "Takes a while"]
    #[test]
    fn pos6_4() {
        let mut board = BoardState::state_from_string_fen(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 4);
        assert_eq!(node_count, 3894594);
    }

    #[ignore = "Takes a while"]
    #[test]
    fn pos6_5() {
        let mut board = BoardState::state_from_string_fen(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ".to_string(),
        );
        let tables = Tables::new();
        let node_count = perft_search(&mut board, &tables, 5);
        assert_eq!(node_count, 164075551);
    }

    #[test]
    fn illegal_pawn_move() {
        let mut board = BoardState::state_from_string_fen(
            "r4q1r/pp6/2nP3P/2PNpbkp/Q4Pp1/6P1/PP6/R3KBNR b KQ f3 0 19".to_string(),
        );
        let tables = Tables::new();
        let move1 = MoveRep::new(
            1 << Tables::E5,
            1 << Tables::F3,
            None,
            PieceType::Pawn,
            Some(PieceType::Pawn),
        );
        let moves = generate(&board, &tables);
        for mv in &moves {
            println!("{}", mv.to_string().unwrap());
        }
        assert!(!moves.contains(&move1));
    }
}
