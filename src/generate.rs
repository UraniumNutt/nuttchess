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
        // if !board.white_in_check(&tables) {
        if true {
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
        // If the king is in check
        // else {
        // If the king is in check, there are three valid responses
        // 1. Attack the attacking piece
        // 2. Block the attacking piece(s)
        // 3. Move the king to safety

        // Try attacking the piece - this can only work if there is only one attacking piece

        // }
    }
    // Black to move
    else {
        // if !board.black_in_check(&tables) {
        if true {
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
        // If the king is in check
        else {
        }
    }

    moves
}

// Generate moves which attack the target
fn generate_attacking_moves(board: &BoardState, tables: &Tables, target: u64) -> Vec<MoveRep> {
    let mut moves = vec![];

    // Get the type of piece of the target
    let target_piece_type = board.get_piece_type(target);

    // Get the mask of pieces which can attack the target
    let mut possible_attacks = match board.white_to_move {
        true => board.white_attacking(&tables, target),
        false => board.black_attacking(&tables, target),
    };

    // If the possible attacks is empty, there are no capturing moves, so return early
    if possible_attacks == 0 {
        return moves;
    }

    // Generate the moves
    while possible_attacks != 0 {
        let start_square = pop_lsb(&mut possible_attacks);
        let piece_type = board.get_piece_type(1 << start_square);
        let mv = MoveRep {
            starting_square: 1 << start_square,
            ending_square: target,
            promotion: None,
            moved_type: piece_type.unwrap(),
            attacked_type: target_piece_type,
        };
        // moves.push(mv);
        let mut foo = board.clone();
        if foo.move_safe_for_king(&tables, &mv) {
            moves.push(mv);
        }
    }

    moves
}

// Generate blocking moves
fn generate_blocking_moves(
    board: &BoardState,
    tables: &Tables,
    protect_target: u64,
    attacking_target: u64,
) -> Vec<MoveRep> {
    let mut moves = vec![];

    // Get the mask of the moves which can be blocked
    let attacking_target_index = attacking_target.trailing_zeros() as u64;
    let protect_target_index = protect_target.trailing_zeros() as u64;
    let piece_type = board.get_piece_type(attacking_target);
    let mut blockable_attack_mask = match piece_type {
        Some(PieceType::Rook) => {
            // The intersection between the attackers mask, and the attack mask as if the attacking piece was on the protected
            // square is the blockable attack mask
            let attackers_mask =
                tables.get_rook_attack(attacking_target_index as usize, board.occupancy());
            let protected_mask =
                tables.get_rook_attack(protect_target_index as usize, board.occupancy());
            attackers_mask & protected_mask
        }
        Some(PieceType::Bishop) => {
            let attackers_mask =
                tables.get_bishop_attack(attacking_target_index as usize, board.occupancy());
            let protected_mask =
                tables.get_bishop_attack(protect_target_index as usize, board.occupancy());
            attackers_mask & protected_mask
        }
        Some(PieceType::Queen) => {
            let attackers_mask_rook =
                tables.get_rook_attack(attacking_target_index as usize, board.occupancy());
            let protected_mask_rook =
                tables.get_rook_attack(protect_target_index as usize, board.occupancy());
            let attackers_mask_bishop =
                tables.get_bishop_attack(attacking_target_index as usize, board.occupancy());
            let protected_mask_bishop =
                tables.get_bishop_attack(protect_target_index as usize, board.occupancy());
            attackers_mask_rook
                & protected_mask_rook
                & attackers_mask_bishop
                & protected_mask_bishop
        }
        _ => 0,
    };

    // If the mask is empty, then there are no moves to block
    if blockable_attack_mask == 0 {
        return moves;
    }

    // Now that we have a mask of the squares that can block the attack, find the moves that attack those squares
    while blockable_attack_mask != 0 {
        let square = pop_lsb(&mut blockable_attack_mask);
        moves.append(generate_attacking_moves(&board, &tables, 1 << square).as_mut());
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

    #[test]
    fn test_gen_attacking_moves_1() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/1P6/8/5B2/3P4/P1P1PPPP/RN1QKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let target = 1 << Tables::C7;

        let expected_move_1 = MoveRep {
            starting_square: 1 << Tables::F4,
            ending_square: 1 << Tables::C7,
            promotion: None,
            moved_type: PieceType::Bishop,
            attacked_type: Some(PieceType::Pawn),
        };

        let expected_move_2 = MoveRep {
            starting_square: 1 << Tables::B6,
            ending_square: 1 << Tables::C7,
            promotion: None,
            moved_type: PieceType::Pawn,
            attacked_type: Some(PieceType::Pawn),
        };

        let results = generate_attacking_moves(&board, &tables, target);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&expected_move_1));
        assert!(results.contains(&expected_move_2));
    }

    #[test]
    fn test_gen_attacking_moves_2() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let results = generate_attacking_moves(&board, &tables, 1 << Tables::E8);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_gen_attacking_moves_3() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/ppp1pppp/8/8/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let target = 1 << Tables::D8;

        let expected_move = MoveRep {
            starting_square: 1 << Tables::D1,
            ending_square: 1 << Tables::D8,
            promotion: None,
            moved_type: PieceType::Queen,
            attacked_type: Some(PieceType::Queen),
        };

        let results = generate_attacking_moves(&board, &tables, target);
        assert_eq!(results.len(), 1);
        assert!(results.contains(&expected_move));
    }

    #[test]
    fn test_gen_attacking_moves_4() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/pp2pppp/8/2p5/3Q4/8/PPP1PPPP/RNB1KBNR b KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let target = 1 << Tables::D4;

        let expected_move_1 = MoveRep {
            starting_square: 1 << Tables::D8,
            ending_square: 1 << Tables::D4,
            promotion: None,
            moved_type: PieceType::Queen,
            attacked_type: Some(PieceType::Queen),
        };

        let expected_move_2 = MoveRep {
            starting_square: 1 << Tables::C5,
            ending_square: 1 << Tables::D4,
            promotion: None,
            moved_type: PieceType::Pawn,
            attacked_type: Some(PieceType::Queen),
        };

        let results = generate_attacking_moves(&board, &tables, target);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&expected_move_1));
        assert!(results.contains(&expected_move_2));
    }

    #[test]
    // fn test_gen_blocking_moves_1() {
    //     let board = BoardState::state_from_string_fen(
    //         "rn1qkbnr/pppppppp/8/5b2/8/2NK4/P1PP1PPP/R1BQ1BNR w kq - 0 1".to_string(),
    //     );
    //     let tables = Tables::new();

    //     let expected_move = MoveRep {
    //         starting_square: 1 << Tables::C3,
    //         ending_square: 1 << Tables::E4,
    //         promotion: None,
    //         moved_type: PieceType::Knight,
    //         attacked_type: None,
    //     };

    //     let results = generate_blocking_moves(&board, &tables, 1 << Tables::D3, 1 << Tables::F5);
    //     assert_eq!(results.len(), 1);
    //     assert!(results.contains(&expected_move));
    // }
    #[test]
    fn test_gen_blocking_moves_2() {}

    #[test]
    fn test_gen_blocking_moves_3() {}

    #[test]
    fn test_gen_blocking_moves_4() {}
}
