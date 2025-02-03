use crate::board::*;
use crate::tables::*;
// Generate a vector of possible moves from the current board state
pub fn generate(board: BoardState, tables: Tables) -> Vec<MoveRep> {
    // move vector
    let mut moves = vec![];

    // white to move
    if board.white_to_move {
        // White pawn moves
        if board.white_pawns != 0 {
            let mut pawn_bb = board.white_pawns;
            // Pushes
            while pawn_bb != 0 {
                let start_square = pop_lsb(&mut pawn_bb);
                let mut pushes = tables.white_pawn_push[start_square];
                while pushes != 0 {
                    let push = MoveRep::new(
                        1 << start_square,
                        1 << pop_lsb(&mut pushes),
                        None,
                        PieceHint::Pawn,
                    );
                    moves.push(push);
                }
            }
            // Attacks
        }
    }
    // Black to move
    else {
    }

    moves
}

#[inline]
// Get and remove the lsb as a square index
fn pop_lsb(bb: &mut u64) -> usize {
    let lsb = bb.trailing_zeros() as usize;
    *bb ^= 1 << lsb;
    lsb
}

#[inline]
// Get the lsb as a square index
fn lsb(bb: u64) -> usize {
    bb.trailing_zeros() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsb() {
        let mut bb = 2;
        let least_sig = lsb(bb);
        assert_eq!(least_sig, 1);
    }

    #[test]
    fn test_pop_lsb() {
        let mut bb = 3;
        let least_sig = pop_lsb(&mut bb);
        assert_eq!(least_sig, 0);
        assert_eq!(bb, 2);
    }
}
