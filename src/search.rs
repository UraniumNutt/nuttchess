use crate::board::*;
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
    // TODO Use bulk counting instead (if depth == 1 return moves.len())
    // For this to work, the move generator needs to only emit legal moves
    // (I think the only puedo legal moves now involve pins)
    // if depth == 0 {
    //     return 1;
    // }
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

    // These can take a while
    // #[test]
    // fn depth_5() {
    //     let mut board = BoardState::starting_state();
    //     let tables = Tables::new();
    //     let node_count = perft_search(&mut board, &tables, 5);
    //     assert_eq!(node_count, 4865609);
    // }

    // #[test]
    // fn white_knight_move_node_count() {
    //     let mut board = BoardState::state_from_string_fen(
    //         "rnbqkbnr/pppppppp/8/8/8/N7/PPPPPPPP/R1BQKBNR b KQkq - 0 1".to_string(),
    //     );
    //     let tables = Tables::new();
    //     let node_count = perft_search(&mut board, &tables, 2);
    //     assert_eq!(node_count, 400);
    // }
}
