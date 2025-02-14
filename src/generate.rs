use crate::board::*;
use crate::tables::*;
// Generate a vector of possible moves from the current board state
pub fn generate(board: &BoardState, tables: &Tables) -> Vec<MoveRep> {
    // move vector
    let mut moves = vec![];

    // Get the occupancys
    let white_occupancy = board.white_occupancy();
    let black_occupancy = board.black_occupancy();
    let occupancy = board.occupancy();

    // white to move
    if board.white_to_move {
        // White pawn moves
        if board.white_pawns != 0 {
            white_pawn_pushes(
                board,
                tables,
                white_occupancy,
                black_occupancy,
                occupancy,
                &mut moves,
            );
        }

        // White Knights
        if board.white_knights != 0 {
            white_knight_attacks(
                board,
                tables,
                white_occupancy,
                black_occupancy,
                occupancy,
                &mut moves,
            );
        }

        // White Rooks
        if board.white_rooks != 0 {
            white_rook_attacks(
                board,
                tables,
                white_occupancy,
                black_occupancy,
                occupancy,
                &mut moves,
            );
        }

        // White Bishops
        if board.white_bishops != 0 {
            white_bishop_attacks(
                board,
                tables,
                white_occupancy,
                black_occupancy,
                occupancy,
                &mut moves,
            );
        }

        // White Queens
        if board.white_queens != 0 {
            white_queen_attacks(
                board,
                tables,
                white_occupancy,
                black_occupancy,
                occupancy,
                &mut moves,
            );
        }

        // White King
        // Remove?
        if board.white_king != 0 {
            white_king_attacks(
                board,
                tables,
                white_occupancy,
                black_occupancy,
                occupancy,
                &mut moves,
            );
        }
    }
    // Black to move
    else {
        // Black pawn moves
        if board.black_pawns != 0 {
            black_pawn_attacks(
                board,
                tables,
                white_occupancy,
                black_occupancy,
                occupancy,
                &mut moves,
            );
        }

        // Black Knights
        if board.black_knights != 0 {
            black_knight_attacks(
                board,
                tables,
                white_occupancy,
                black_occupancy,
                occupancy,
                &mut moves,
            );
        }

        // Black Rooks
        if board.black_rooks != 0 {
            black_rook_attacks(
                board,
                tables,
                white_occupancy,
                black_occupancy,
                occupancy,
                &mut moves,
            );
        }

        // Black Bishops
        if board.black_bishops != 0 {
            black_bishop_attacks(
                board,
                tables,
                white_occupancy,
                black_occupancy,
                occupancy,
                &mut moves,
            );
        }

        // Black Queens
        if board.black_queens != 0 {
            black_queen_attacks(
                board,
                tables,
                white_occupancy,
                black_occupancy,
                occupancy,
                &mut moves,
            );
        }

        // Black King
        // Remove?
        if board.black_king != 0 {
            black_king_attacks(
                board,
                tables,
                white_occupancy,
                black_occupancy,
                occupancy,
                &mut moves,
            );
        }
    }

    moves
}

#[inline]
fn white_pawn_pushes(
    board: &BoardState,
    tables: &Tables,
    white_occupancy: u64,
    black_occupancy: u64,
    occupancy: u64,
    moves: &mut Vec<MoveRep>,
) {
    let mut pawn_bb = board.white_pawns;
    // White Pawn Pushes
    while pawn_bb != 0 {
        let start_square = pop_lsb(&mut pawn_bb);
        let mut pushes = tables.white_pawn_push[start_square];
        while pushes != 0 {
            let end_square = 1 << pop_lsb(&mut pushes);
            let attacked_type = board.get_piece_type(end_square);
            // Check that a double push does not skip over a piece
            if (end_square >> 16) == (1u64 << start_square) as u64
                && (end_square >> 8) & occupancy != 0
            {
                continue;
            }
            if (end_square & occupancy) == 0 {
                let push = MoveRep::new(
                    1 << start_square,
                    end_square,
                    None,
                    PieceType::Pawn,
                    attacked_type,
                );
                moves.push(push);
            }
        }
    }
    // White Pawn Attacks
    pawn_bb = board.white_pawns;
    while pawn_bb != 0 {
        let start_square = pop_lsb(&mut pawn_bb);
        let mut attacks = tables.white_pawn_attacks[start_square];
        while attacks != 0 {
            let end_square = 1 << pop_lsb(&mut attacks);
            let attacked_type = board.get_piece_type(end_square);
            if (end_square & black_occupancy) != 0 {
                let attack = MoveRep::new(
                    1 << start_square,
                    end_square,
                    None,
                    PieceType::Pawn,
                    attacked_type,
                );
                moves.push(attack);
            }
        }
    }
}

#[inline]
fn white_pawn_attacks(
    board: &BoardState,
    tables: &Tables,
    white_occupancy: u64,
    black_occupancy: u64,
    occupancy: u64,
    moves: &mut Vec<MoveRep>,
) {
    todo!();
}

#[inline]
fn white_knight_attacks(
    board: &BoardState,
    tables: &Tables,
    white_occupancy: u64,
    black_occupancy: u64,
    occupancy: u64,
    moves: &mut Vec<MoveRep>,
) {
    let mut knight_bb = board.white_knights;
    while knight_bb != 0 {
        let start_square = pop_lsb(&mut knight_bb);
        let mut attacks = tables.knight_attacks[start_square];
        while attacks != 0 {
            let end_square = 1 << pop_lsb(&mut attacks);
            let attacked_type = board.get_piece_type(end_square);
            if (end_square & white_occupancy) == 0 {
                let attack = MoveRep::new(
                    1 << start_square,
                    end_square,
                    None,
                    PieceType::Knight,
                    attacked_type,
                );
                moves.push(attack);
            }
        }
    }
}

#[inline]
fn white_rook_attacks(
    board: &BoardState,
    tables: &Tables,
    white_occupancy: u64,
    black_occupancy: u64,
    occupancy: u64,
    moves: &mut Vec<MoveRep>,
) {
    let mut rook_bb = board.white_rooks;
    while rook_bb != 0 {
        let start_square = pop_lsb(&mut rook_bb);
        let mut attacks = tables.get_rook_attack(start_square, occupancy) & !white_occupancy;
        while attacks != 0 {
            let end_square = pop_lsb(&mut attacks);
            let attacked_type = board.get_piece_type(1 << end_square);
            let attack = MoveRep::new(
                1 << start_square,
                1 << end_square,
                None,
                PieceType::Rook,
                attacked_type,
            );
            moves.push(attack);
        }
    }
}

#[inline]
fn white_bishop_attacks(
    board: &BoardState,
    tables: &Tables,
    white_occupancy: u64,
    black_occupancy: u64,
    occupancy: u64,
    moves: &mut Vec<MoveRep>,
) {
    let mut bishop_bb = board.white_bishops;
    while bishop_bb != 0 {
        let start_square = pop_lsb(&mut bishop_bb);
        let mut attacks = tables.get_bishop_attack(start_square, occupancy) & !white_occupancy;
        while attacks != 0 {
            let end_square = pop_lsb(&mut attacks);
            let attacked_type = board.get_piece_type(1 << end_square);
            let attack = MoveRep {
                starting_square: 1 << start_square,
                ending_square: 1 << end_square,
                promotion: None,
                moved_type: PieceType::Bishop,
                attacked_type: attacked_type,
            };
            moves.push(attack);
        }
    }
}

#[inline]
fn white_queen_attacks(
    board: &BoardState,
    tables: &Tables,
    white_occupancy: u64,
    black_occupancy: u64,
    occupancy: u64,
    moves: &mut Vec<MoveRep>,
) {
    // Rook like
    let mut rook_bb = board.white_queens;
    while rook_bb != 0 {
        let start_square = pop_lsb(&mut rook_bb);
        let mut attacks = tables.get_rook_attack(start_square, occupancy) & !white_occupancy;
        while attacks != 0 {
            let end_square = pop_lsb(&mut attacks);
            let attacked_type = board.get_piece_type(1 << end_square);
            let attack = MoveRep::new(
                1 << start_square,
                1 << end_square,
                None,
                PieceType::Queen,
                attacked_type,
            );
            moves.push(attack);
        }
    }

    // Bishop like
    let mut bishop_bb = board.white_queens;
    while bishop_bb != 0 {
        let start_square = pop_lsb(&mut bishop_bb);
        let mut attacks = tables.get_bishop_attack(start_square, occupancy) & !white_occupancy;
        while attacks != 0 {
            let end_square = pop_lsb(&mut attacks);
            let attacked_type = board.get_piece_type(1 << end_square);
            let attack = MoveRep {
                starting_square: 1 << start_square,
                ending_square: 1 << end_square,
                promotion: None,
                moved_type: PieceType::Queen,
                attacked_type: attacked_type,
            };
            moves.push(attack);
        }
    }
}

#[inline]
fn white_king_attacks(
    board: &BoardState,
    tables: &Tables,
    white_occupancy: u64,
    black_occupancy: u64,
    occupancy: u64,
    moves: &mut Vec<MoveRep>,
) {
    let mut king_bb = board.white_king;
    while king_bb != 0 {
        let start_square = pop_lsb(&mut king_bb) as u64;
        let mut attacks = tables.king_attacks[start_square as usize] & !white_occupancy;
        while attacks != 0 {
            let end_square = pop_lsb(&mut attacks) as u64;
            let attacked_type = board.get_piece_type(1 << end_square);
            let black_attack_mask = board.black_attack_mask(&tables);
            let attack = MoveRep {
                starting_square: 1 << start_square,
                ending_square: 1 << end_square,
                promotion: None,
                moved_type: PieceType::King,
                attacked_type: attacked_type,
            };
            if black_attack_mask & 1 << end_square == 0 {
                moves.push(attack);
            }
        }
    }
}

#[inline]
fn black_pawn_attacks(
    board: &BoardState,
    tables: &Tables,
    white_occupancy: u64,
    black_occupancy: u64,
    occupancy: u64,
    moves: &mut Vec<MoveRep>,
) {
    let mut pawn_bb = board.black_pawns;
    // Black Pawn Pushes
    while pawn_bb != 0 {
        let start_square = pop_lsb(&mut pawn_bb);
        let mut pushes = tables.black_pawn_push[start_square];
        while pushes != 0 {
            let end_square = 1 << pop_lsb(&mut pushes);
            let attacked_type = board.get_piece_type(end_square);
            // Check that a double push does not skip over a piece
            if (end_square << 16) == (1u64 << start_square) as u64
                && (end_square << 8) & occupancy != 0
            {
                continue;
            }
            if (end_square & occupancy) == 0 {
                let push = MoveRep::new(
                    1 << start_square,
                    end_square,
                    None,
                    PieceType::Pawn,
                    attacked_type,
                );
                moves.push(push);
            }
        }
    }
    // Black Pawn Attacks
    pawn_bb = board.black_pawns;
    while pawn_bb != 0 {
        let start_square = pop_lsb(&mut pawn_bb);
        let mut attacks = tables.black_pawn_attacks[start_square];
        while attacks != 0 {
            let end_square = 1 << pop_lsb(&mut attacks);
            let attacked_type = board.get_piece_type(end_square);
            if (end_square & white_occupancy) != 0 {
                let attack = MoveRep::new(
                    1 << start_square,
                    end_square,
                    None,
                    PieceType::Pawn,
                    attacked_type,
                );
                moves.push(attack);
            }
        }
    }
}

#[inline]
fn black_knight_attacks(
    board: &BoardState,
    tables: &Tables,
    white_occupancy: u64,
    black_occupancy: u64,
    occupancy: u64,
    moves: &mut Vec<MoveRep>,
) {
    let mut knight_bb = board.black_knights;
    while knight_bb != 0 {
        let start_square = pop_lsb(&mut knight_bb);
        let mut attacks = tables.knight_attacks[start_square];
        while attacks != 0 {
            let end_square = 1 << pop_lsb(&mut attacks);
            let attacked_type = board.get_piece_type(end_square);
            if (end_square & black_occupancy) == 0 {
                let attack = MoveRep::new(
                    1 << start_square,
                    end_square,
                    None,
                    PieceType::Knight,
                    attacked_type,
                );
                moves.push(attack);
            }
        }
    }
}

#[inline]
fn black_rook_attacks(
    board: &BoardState,
    tables: &Tables,
    white_occupancy: u64,
    black_occupancy: u64,
    occupancy: u64,
    moves: &mut Vec<MoveRep>,
) {
    let mut rook_bb = board.black_rooks;
    while rook_bb != 0 {
        let start_square = pop_lsb(&mut rook_bb);
        let mut attacks = tables.get_rook_attack(start_square, occupancy) & !black_occupancy;
        while attacks != 0 {
            let end_square = pop_lsb(&mut attacks);
            let attacked_type = board.get_piece_type(1 << end_square);
            let attack = MoveRep::new(
                1 << start_square,
                1 << end_square,
                None,
                PieceType::Rook,
                attacked_type,
            );
            moves.push(attack);
        }
    }
}

#[inline]
fn black_bishop_attacks(
    board: &BoardState,
    tables: &Tables,
    white_occupancy: u64,
    black_occupancy: u64,
    occupancy: u64,
    moves: &mut Vec<MoveRep>,
) {
    let mut bishop_bb = board.black_bishops;
    while bishop_bb != 0 {
        let start_square = pop_lsb(&mut bishop_bb);
        let mut attacks = tables.get_bishop_attack(start_square, occupancy) & !black_occupancy;
        while attacks != 0 {
            let end_square = pop_lsb(&mut attacks);
            let attacked_type = board.get_piece_type(1 << end_square);
            let attack = MoveRep {
                starting_square: 1 << start_square,
                ending_square: 1 << end_square,
                promotion: None,
                moved_type: PieceType::Bishop,
                attacked_type: attacked_type,
            };
            moves.push(attack);
        }
    }
}

#[inline]
fn black_queen_attacks(
    board: &BoardState,
    tables: &Tables,
    white_occupancy: u64,
    black_occupancy: u64,
    occupancy: u64,
    moves: &mut Vec<MoveRep>,
) {
    let mut rook_bb = board.black_queens;
    while rook_bb != 0 {
        let start_square = pop_lsb(&mut rook_bb);
        let mut attacks = tables.get_rook_attack(start_square, occupancy) & !black_occupancy;
        while attacks != 0 {
            let end_square = pop_lsb(&mut attacks);
            let attacked_type = board.get_piece_type(1 << end_square);
            let attack = MoveRep::new(
                1 << start_square,
                1 << end_square,
                None,
                PieceType::Queen,
                attacked_type,
            );
            moves.push(attack);
        }
    }

    // Bishop like
    let mut bishop_bb = board.black_queens;
    while bishop_bb != 0 {
        let start_square = pop_lsb(&mut bishop_bb);
        let mut attacks = tables.get_bishop_attack(start_square, occupancy) & !black_occupancy;
        while attacks != 0 {
            let end_square = pop_lsb(&mut attacks);
            let attacked_type = board.get_piece_type(1 << end_square);
            let attack = MoveRep {
                starting_square: 1 << start_square,
                ending_square: 1 << end_square,
                promotion: None,
                moved_type: PieceType::Queen,
                attacked_type: attacked_type,
            };
            moves.push(attack);
        }
    }
}

#[inline]
fn black_king_attacks(
    board: &BoardState,
    tables: &Tables,
    white_occupancy: u64,
    black_occupancy: u64,
    occupancy: u64,
    moves: &mut Vec<MoveRep>,
) {
    let mut king_bb = board.black_king;
    while king_bb != 0 {
        let start_square = pop_lsb(&mut king_bb) as u64;
        let mut attacks = tables.king_attacks[start_square as usize] & !black_occupancy;
        while attacks != 0 {
            let end_square = pop_lsb(&mut attacks) as u64;
            let attacked_type = board.get_piece_type(1 << end_square);
            let white_attack_mask = board.white_attack_mask(&tables);
            let attack = MoveRep {
                starting_square: 1 << start_square,
                ending_square: 1 << end_square,
                promotion: None,
                moved_type: PieceType::King,
                attacked_type: attacked_type,
            };
            if white_attack_mask & 1 << end_square == 0 {
                moves.push(attack);
            }
        }
    }
}

#[inline]
// Get and remove the lsb as a square index
pub fn pop_lsb(bb: &mut u64) -> usize {
    let lsb = bb.trailing_zeros() as usize;
    *bb ^= 1 << lsb;
    lsb
}

#[inline]
// Get the lsb as a square index
pub fn lsb(bb: u64) -> usize {
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
