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
        }
    }
    println!("\n{}", total_node_count);
}

fn perft_search(board: &mut BoardState, tables: &Tables, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }
    let mut node_count = 0;
    let moves = generate(board, &tables);
    for mv in moves {
        board.make(&mv);
        node_count += perft_search(board, &tables, depth - 1);
        board.unmake(&mv);
    }
    return node_count;
}
