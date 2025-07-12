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
use crate::{
    board::{BoardState, MoveRep, PieceType, Promotion},
    tables::Tables,
};

/// Generate a vector of possible moves from the current board state
pub fn generate(board: &BoardState, tables: &Tables) -> Vec<MoveRep> {
    // Move list
    let mut moves = Vec::with_capacity(256);

    // Get the sides to moves king
    let king = match board.white_to_move {
        true => board.white_king,
        false => board.black_king,
    };

    // Get the pinned pieces
    let pinned_pieces = board.pin_mask(tables, king, board.white_to_move);

    // white to move
    if board.white_to_move {
        if !board.white_in_check(tables) {
            // White pawn moves
            white_pawn_moves(board, tables, pinned_pieces, king, &mut moves);

            // White Knights
            white_knight_attacks(board, tables, pinned_pieces, king, &mut moves);

            // White Rooks
            white_rook_attacks(board, tables, pinned_pieces, king, &mut moves);

            // White Bishops
            white_bishop_attacks(board, tables, pinned_pieces, king, &mut moves);

            // White Queens
            white_queen_attacks(board, tables, pinned_pieces, king, &mut moves);

            // White King
            white_king_attacks(board, tables, pinned_pieces, king, &mut moves);

            // White castle
            if board.white_queenside_castle_rights || board.white_kingside_castle_rights {
                if board.white_queenside_castle_rights
                    && board.black_attacking(tables, board.white_king) == 0
                    && board.black_attacking(tables, board.white_king << 1) == 0
                    && board.black_attacking(tables, board.white_king << 2) == 0
                    && board.occupancy() & 0x70 == 0
                {
                    let mv = MoveRep::new(
                        1 << Tables::E1,
                        1 << Tables::C1,
                        Some(Promotion::Castle),
                        PieceType::King,
                        None,
                    );
                    moves.push(mv);
                }
                if board.white_kingside_castle_rights
                    && board.black_attacking(tables, board.white_king) == 0
                    && board.black_attacking(tables, board.white_king >> 1) == 0
                    && board.black_attacking(tables, board.white_king >> 2) == 0
                    && board.occupancy() & 0x6 == 0
                {
                    let mv = MoveRep::new(
                        1 << Tables::E1,
                        1 << Tables::G1,
                        Some(Promotion::Castle),
                        PieceType::King,
                        None,
                    );
                    moves.push(mv);
                }
            }
        }
        // If the king is in check
        else {
            // If the king is in check, there are three valid responses
            // 1. Attack the attacking piece
            // 2. Block the attacking piece(s)
            // 3. Move the king to safety

            // Try attacking and blocking the piece - this can only work if there is only one attacking piece
            if board.black_attacking(tables, board.white_king).count_ones() == 1 {
                let target = board.black_attacking(tables, board.white_king);
                moves.append(&mut generate_attacking_moves(board, tables, target));
                // Also try to generate en passant moves which attack the target

                if board.en_passant_target >> 8 == target {
                    let en_passant_index = board.en_passant_target.trailing_zeros() as usize;
                    let mut attacking_mask =
                        tables.black_pawn_attacks[en_passant_index] & board.white_pawns;
                    while attacking_mask != 0 {
                        let attacking_index = pop_lsb(&mut attacking_mask);
                        let mv = MoveRep::new(
                            1 << attacking_index,
                            board.en_passant_target,
                            None,
                            PieceType::Pawn,
                            Some(PieceType::Pawn),
                        );
                        if board.pin_safe(tables, board.white_king, &mv) {
                            moves.push(mv);
                        }
                    }
                }
                moves.append(&mut generate_blocking_moves(
                    board,
                    tables,
                    board.white_king,
                    target,
                ));
            }
            // Now try moving the king to safety
            moves.append(&mut move_king_to_safety(board, tables));
        }
    }
    // Black to move
    else if !board.black_in_check(tables) {
        // Black pawn moves
        black_pawn_moves(board, tables, pinned_pieces, king, &mut moves);

        // Black Knights
        black_knight_attacks(board, tables, pinned_pieces, king, &mut moves);

        // Black Rooks
        black_rook_attacks(board, tables, pinned_pieces, king, &mut moves);

        // Black Bishops
        black_bishop_attacks(board, tables, pinned_pieces, king, &mut moves);

        // Black Queens
        black_queen_attacks(board, tables, pinned_pieces, king, &mut moves);

        // Black King
        black_king_attacks(board, tables, pinned_pieces, king, &mut moves);

        // Black castling
        if board.black_queenside_castle_rights || board.black_kingside_castle_rights {
            if board.black_queenside_castle_rights
                && board.white_attacking(tables, board.black_king) == 0
                && board.white_attacking(tables, board.black_king << 1) == 0
                && board.white_attacking(tables, board.black_king << 2) == 0
                && board.occupancy() & 0x7000000000000000 == 0
            {
                let mv = MoveRep::new(
                    1 << Tables::E8,
                    1 << Tables::C8,
                    Some(Promotion::Castle),
                    PieceType::King,
                    None,
                );
                moves.push(mv);
            }
            if board.black_kingside_castle_rights
                && board.white_attacking(tables, board.black_king) == 0
                && board.white_attacking(tables, board.black_king >> 1) == 0
                && board.white_attacking(tables, board.black_king >> 2) == 0
                && board.occupancy() & 0x600000000000000 == 0
            {
                let mv = MoveRep::new(
                    1 << Tables::E8,
                    1 << Tables::G8,
                    Some(Promotion::Castle),
                    PieceType::King,
                    None,
                );
                moves.push(mv);
            }
        }
    }
    // If the king is in check
    else {
        // If the king is in check, there are three valid responses
        // 1. Attack the attacking piece
        // 2. Block the attacking piece(s)
        // 3. Move the king to safety

        // Try attacking and blocking the piece - this can only work if there is only one attacking piece
        if board.white_attacking(tables, board.black_king).count_ones() == 1 {
            let target = board.white_attacking(tables, board.black_king);
            moves.append(&mut generate_attacking_moves(board, tables, target));
            // Also try to generate en passant moves which attack the target

            if board.en_passant_target << 8 == target {
                let en_passant_index = board.en_passant_target.trailing_zeros() as usize;
                let mut attacking_mask =
                    tables.white_pawn_attacks[en_passant_index] & board.black_pawns;
                while attacking_mask != 0 {
                    let attacking_index = pop_lsb(&mut attacking_mask);
                    let mv = MoveRep::new(
                        1 << attacking_index,
                        board.en_passant_target,
                        None,
                        PieceType::Pawn,
                        Some(PieceType::Pawn),
                    );
                    if board.pin_safe(tables, board.black_king, &mv) {
                        moves.push(mv);
                    }
                }
            }
            moves.append(&mut generate_blocking_moves(
                board,
                tables,
                board.black_king,
                target,
            ));
        }
        // Now try moving the king to safety
        moves.append(&mut move_king_to_safety(board, tables));
    }

    moves
}

// Generate moves which attack the target
pub fn generate_attacking_moves(board: &BoardState, tables: &Tables, target: u64) -> Vec<MoveRep> {
    let mut moves = Vec::with_capacity(256);
    // Get the pinned pieces
    let pinned_pieces = match board.white_to_move {
        true => board.pin_mask(tables, board.white_king, board.white_to_move),
        false => board.pin_mask(tables, board.black_king, board.white_to_move),
    };
    // Get the sides to moves king
    let king = match board.white_to_move {
        true => board.white_king,
        false => board.black_king,
    };

    // Get the type of piece of the target
    let target_piece_type = board.get_piece_type(target);

    // Get the mask of pieces which can attack the target
    let mut possible_attacks = match board.white_to_move {
        true => board.white_attacking(tables, target),
        false => board.black_attacking(tables, target),
    };
    // If the possible attacks is empty, there are no capturing moves, so return early
    if possible_attacks == 0 {
        return moves;
    }

    // Generate the moves
    while possible_attacks != 0 {
        let start_square = pop_lsb(&mut possible_attacks);
        let piece_type = board.get_piece_type(1 << start_square);

        // NOTE We dont generate en passant attacks here because a diffrent function handles that logic

        let mv = MoveRep {
            starting_square: 1 << start_square,
            ending_square: target,
            promotion: None,
            moved_type: piece_type.unwrap(),
            attacked_type: target_piece_type,
        };
        // To prevent move duplication, dont produce king attacks here; that will be done in move_king_to_safety
        if mv.moved_type == PieceType::King {
            continue;
        }
        if mv.starting_square & pinned_pieces == 0 || board.pin_safe(tables, king, &mv) {
            if mv.moved_type == PieceType::Pawn {
                if board.white_to_move && mv.ending_square & Tables::RANK_8 != 0 {
                    // White promotion
                    let mut queen_promotion = mv;
                    queen_promotion.promotion = Some(Promotion::Queen);
                    let mut rook_promotion = mv;
                    rook_promotion.promotion = Some(Promotion::Rook);
                    let mut bishop_promotion = mv;
                    bishop_promotion.promotion = Some(Promotion::Bishop);
                    let mut knight_promotion = mv;
                    knight_promotion.promotion = Some(Promotion::Knight);
                    moves.push(queen_promotion);
                    moves.push(rook_promotion);
                    moves.push(bishop_promotion);
                    moves.push(knight_promotion);
                }
                if !board.white_to_move && mv.ending_square & Tables::RANK_1 != 0 {
                    // Black promotion
                    let mut queen_promotion = mv;
                    queen_promotion.promotion = Some(Promotion::Queen);
                    let mut rook_promotion = mv;
                    rook_promotion.promotion = Some(Promotion::Rook);
                    let mut bishop_promotion = mv;
                    bishop_promotion.promotion = Some(Promotion::Bishop);
                    let mut knight_promotion = mv;
                    knight_promotion.promotion = Some(Promotion::Knight);
                    moves.push(queen_promotion);
                    moves.push(rook_promotion);
                    moves.push(bishop_promotion);
                    moves.push(knight_promotion);
                }
                if (mv.ending_square & (Tables::RANK_1 | Tables::RANK_8)) == 0 {
                    moves.push(mv);
                }
            } else {
                moves.push(mv);
            }
        }
    }

    moves
}

// Generates moves that block (do not capture) the target. Similar to generate_attacking_moves, but with pawn pushes instead of attacks
pub fn generate_target_blocking(
    board: &BoardState,
    tables: &Tables,
    target: u64,
    protect_target: u64,
) -> Vec<MoveRep> {
    let mut moves = Vec::with_capacity(256);

    // Get the type of piece of the target
    let target_piece_type = board.get_piece_type(target);
    // Get the pinned pieces
    let pinned_pieces = match board.white_to_move {
        true => board.pin_mask(tables, board.white_king, board.white_to_move),
        false => board.pin_mask(tables, board.black_king, board.white_to_move),
    };
    // Get the sides to moves king
    let king = match board.white_to_move {
        true => board.white_king,
        false => board.black_king,
    };

    // Get the mask of pieces which can attack the target
    let mut possible_attacks = match board.white_to_move {
        true => board.white_blocking(tables, target),
        false => board.black_blocking(tables, target),
    };

    // Remove any attacks which use the protect piece, it cant protect itself
    possible_attacks &= !protect_target;

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
        if pinned_pieces & mv.starting_square == 0 || board.pin_safe(tables, king, &mv) {
            moves.push(mv);
        }
    }

    moves
}

// Generate blocking moves
pub fn generate_blocking_moves(
    board: &BoardState,
    tables: &Tables,
    protect_target: u64,
    attacking_target: u64,
) -> Vec<MoveRep> {
    let mut moves = Vec::with_capacity(256);
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
            // We need to find if the ray is rook, or bishop like
            if attackers_mask_rook & protect_target != 0 {
                // Rook like
                attackers_mask_rook & protected_mask_rook
            } else {
                // Bishop like
                attackers_mask_bishop & protected_mask_bishop
            }
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
        moves.append(generate_target_blocking(board, tables, 1 << square, protect_target).as_mut());
    }

    moves
}

// Generate moves which move the king to safety; also includes moves which attack an adjacent target
pub fn move_king_to_safety(board: &BoardState, tables: &Tables) -> Vec<MoveRep> {
    let mut moves = Vec::with_capacity(256);

    // Get the king position
    let king = match board.white_to_move {
        true => board.white_king,
        false => board.black_king,
    };

    let king_index = king.trailing_zeros();
    let mut safe_squares = match board.white_to_move {
        true => {
            tables.king_attacks[king_index as usize]
                & !board.black_attack_mask_with_transparency(tables, king)
                & !board.white_occupancy()
        }
        false => {
            tables.king_attacks[king_index as usize]
                & !board.white_attack_mask_with_transparency(tables, king)
                & !board.black_occupancy()
        }
    };
    while safe_squares != 0 {
        let end_square = pop_lsb(&mut safe_squares);
        let attacked_type = board.get_piece_type(1 << end_square);
        let mv = MoveRep::new(king, 1 << end_square, None, PieceType::King, attacked_type);
        moves.push(mv);
    }

    moves
}

fn white_pawn_moves(
    board: &BoardState,
    tables: &Tables,
    pinned_pieces: u64,
    king: u64,
    moves: &mut Vec<MoveRep>,
) {
    let black_occupancy = board.black_occupancy();
    let occupancy = board.occupancy();

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
                if push.starting_square & pinned_pieces == 0 || board.pin_safe(tables, king, &push)
                {
                    if end_square & Tables::RANK_8 == 0 {
                        moves.push(push);
                    } else {
                        // Promotion
                        let mut queen_promotion = push;
                        queen_promotion.promotion = Some(Promotion::Queen);
                        let mut rook_promotion = push;
                        rook_promotion.promotion = Some(Promotion::Rook);
                        let mut bishop_promotion = push;
                        bishop_promotion.promotion = Some(Promotion::Bishop);
                        let mut knight_promotion = push;
                        knight_promotion.promotion = Some(Promotion::Knight);
                        moves.push(queen_promotion);
                        moves.push(rook_promotion);
                        moves.push(bishop_promotion);
                        moves.push(knight_promotion);
                    }
                }
            }
        }
    }
    // White Pawn Attacks
    pawn_bb = board.white_pawns;
    while pawn_bb != 0 {
        let start_square = pop_lsb(&mut pawn_bb);
        let mut attacks = tables.white_pawn_attacks[start_square] & black_occupancy;
        while attacks != 0 {
            let end_square = 1 << pop_lsb(&mut attacks);
            let attacked_type = board.get_piece_type(end_square);
            let attack = MoveRep::new(
                1 << start_square,
                end_square,
                None,
                PieceType::Pawn,
                attacked_type,
            );
            if attack.starting_square & pinned_pieces == 0 || board.pin_safe(tables, king, &attack)
            {
                if end_square & Tables::RANK_8 == 0 {
                    moves.push(attack);
                } else {
                    // Promotion
                    let mut queen_promotion = attack;
                    queen_promotion.promotion = Some(Promotion::Queen);
                    let mut rook_promotion = attack;
                    rook_promotion.promotion = Some(Promotion::Rook);
                    let mut bishop_promotion = attack;
                    bishop_promotion.promotion = Some(Promotion::Bishop);
                    let mut knight_promotion = attack;
                    knight_promotion.promotion = Some(Promotion::Knight);
                    moves.push(queen_promotion);
                    moves.push(rook_promotion);
                    moves.push(bishop_promotion);
                    moves.push(knight_promotion);
                }
            }
        }
    }
    // White Pawn En Passant Attacks
    // Get relevent white pawns (look 'backward' so use opposite color in attack lookup)
    if board.en_passant_target != 0 {
        pawn_bb = board.white_pawns
            & tables.black_pawn_attacks[board.en_passant_target.trailing_zeros() as usize];
        while pawn_bb != 0 {
            let start_square = pop_lsb(&mut pawn_bb);
            let mut attacks = tables.white_pawn_attacks[start_square] & board.en_passant_target;
            while attacks != 0 {
                let end_square = 1 << pop_lsb(&mut attacks);
                // We know its a pawn
                let attacked_type = PieceType::Pawn;
                let attack = MoveRep::new(
                    1 << start_square,
                    end_square,
                    None,
                    attacked_type,
                    Some(PieceType::Pawn),
                );
                // This uses AND instead of OR to prevent discoverd en passant attacks
                if attack.starting_square & pinned_pieces == 0
                    && board.pin_safe(tables, king, &attack)
                {
                    moves.push(attack);
                }
            }
        }
    }
}

fn white_knight_attacks(
    board: &BoardState,
    tables: &Tables,
    pinned_pieces: u64,
    king: u64,
    moves: &mut Vec<MoveRep>,
) {
    let white_occupancy = board.white_occupancy();
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
                if attack.starting_square & pinned_pieces == 0
                    || board.pin_safe(tables, king, &attack)
                {
                    moves.push(attack);
                }
            }
        }
    }
}

fn white_rook_attacks(
    board: &BoardState,
    tables: &Tables,
    pinned_pieces: u64,
    king: u64,
    moves: &mut Vec<MoveRep>,
) {
    let white_occupancy = board.white_occupancy();
    let occupancy = board.occupancy();

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
            if attack.starting_square & pinned_pieces == 0 || board.pin_safe(tables, king, &attack)
            {
                moves.push(attack);
            }
        }
    }
}

fn white_bishop_attacks(
    board: &BoardState,
    tables: &Tables,
    pinned_pieces: u64,
    king: u64,
    moves: &mut Vec<MoveRep>,
) {
    let white_occupancy = board.white_occupancy();
    let occupancy = board.occupancy();
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
                attacked_type,
            };
            if attack.starting_square & pinned_pieces == 0 || board.pin_safe(tables, king, &attack)
            {
                moves.push(attack);
            }
        }
    }
}

fn white_queen_attacks(
    board: &BoardState,
    tables: &Tables,
    pinned_pieces: u64,
    king: u64,
    moves: &mut Vec<MoveRep>,
) {
    let white_occupancy = board.white_occupancy();
    let occupancy = board.occupancy();
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
            if attack.starting_square & pinned_pieces == 0 || board.pin_safe(tables, king, &attack)
            {
                moves.push(attack);
            }
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
                attacked_type,
            };
            if attack.starting_square & pinned_pieces == 0 || board.pin_safe(tables, king, &attack)
            {
                moves.push(attack);
            }
        }
    }
}

fn white_king_attacks(
    board: &BoardState,
    tables: &Tables,
    pinned_pieces: u64,
    king: u64,
    moves: &mut Vec<MoveRep>,
) {
    let white_occupancy = board.white_occupancy();
    let mut king_bb = board.white_king;
    while king_bb != 0 {
        let start_square = pop_lsb(&mut king_bb) as u64;
        let mut attacks = tables.king_attacks[start_square as usize] & !white_occupancy;
        while attacks != 0 {
            let end_square = pop_lsb(&mut attacks) as u64;
            let attacked_type = board.get_piece_type(1 << end_square);
            let black_attack_mask = board.black_attack_mask(tables);
            let attack = MoveRep {
                starting_square: 1 << start_square,
                ending_square: 1 << end_square,
                promotion: None,
                moved_type: PieceType::King,
                attacked_type,
            };
            if black_attack_mask & 1 << end_square == 0
                && (attack.starting_square & pinned_pieces == 0
                    || board.pin_safe(tables, king, &attack))
            {
                moves.push(attack);
            }
        }
    }
}

fn black_pawn_moves(
    board: &BoardState,
    tables: &Tables,
    pinned_pieces: u64,
    king: u64,
    moves: &mut Vec<MoveRep>,
) {
    let white_occupancy = board.white_occupancy();
    let occupancy = board.occupancy();

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
                if push.starting_square & pinned_pieces == 0 || board.pin_safe(tables, king, &push)
                {
                    if end_square & Tables::RANK_1 == 0 {
                        moves.push(push);
                    } else {
                        // Promotion
                        let mut queen_promotion = push;
                        queen_promotion.promotion = Some(Promotion::Queen);
                        let mut rook_promotion = push;
                        rook_promotion.promotion = Some(Promotion::Rook);
                        let mut bishop_promotion = push;
                        bishop_promotion.promotion = Some(Promotion::Bishop);
                        let mut knight_promotion = push;
                        knight_promotion.promotion = Some(Promotion::Knight);
                        moves.push(queen_promotion);
                        moves.push(rook_promotion);
                        moves.push(bishop_promotion);
                        moves.push(knight_promotion);
                    }
                }
            }
        }
    }
    // Black Pawn Attacks
    pawn_bb = board.black_pawns;
    while pawn_bb != 0 {
        let start_square = pop_lsb(&mut pawn_bb);
        let mut attacks = tables.black_pawn_attacks[start_square] & white_occupancy;
        while attacks != 0 {
            let end_square = 1 << pop_lsb(&mut attacks);
            let attacked_type = board.get_piece_type(end_square);
            let attack = MoveRep::new(
                1 << start_square,
                end_square,
                None,
                PieceType::Pawn,
                attacked_type,
            );
            if attack.starting_square & pinned_pieces == 0 || board.pin_safe(tables, king, &attack)
            {
                if end_square & Tables::RANK_1 == 0 {
                    moves.push(attack);
                } else {
                    // Promotion
                    let mut queen_promotion = attack;
                    queen_promotion.promotion = Some(Promotion::Queen);
                    let mut rook_promotion = attack;
                    rook_promotion.promotion = Some(Promotion::Rook);
                    let mut bishop_promotion = attack;
                    bishop_promotion.promotion = Some(Promotion::Bishop);
                    let mut knight_promotion = attack;
                    knight_promotion.promotion = Some(Promotion::Knight);
                    moves.push(queen_promotion);
                    moves.push(rook_promotion);
                    moves.push(bishop_promotion);
                    moves.push(knight_promotion);
                }
            }
        }
    }

    // Black Pawn En Passant Attacks
    // Get relevent black pawns (look 'backward' so use opposite color in attack lookup)
    if board.en_passant_target != 0 {
        pawn_bb = board.black_pawns
            & tables.white_pawn_attacks[board.en_passant_target.trailing_zeros() as usize];
        while pawn_bb != 0 {
            let start_square = pop_lsb(&mut pawn_bb);
            let mut attacks = tables.black_pawn_attacks[start_square] & board.en_passant_target;
            while attacks != 0 {
                let end_square = 1 << pop_lsb(&mut attacks);
                // We know its a pawn
                let attacked_type = PieceType::Pawn;
                let attack = MoveRep::new(
                    1 << start_square,
                    end_square,
                    None,
                    attacked_type,
                    Some(PieceType::Pawn),
                );
                // This uses AND instead of OR to prevent discoverd en passant attacks
                if attack.starting_square & pinned_pieces == 0
                    && board.pin_safe(tables, king, &attack)
                {
                    moves.push(attack);
                }
            }
        }
    }
}

fn black_knight_attacks(
    board: &BoardState,
    tables: &Tables,
    pinned_pieces: u64,
    king: u64,
    moves: &mut Vec<MoveRep>,
) {
    let black_occupancy = board.black_occupancy();
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
                if attack.starting_square & pinned_pieces == 0
                    || board.pin_safe(tables, king, &attack)
                {
                    moves.push(attack);
                }
            }
        }
    }
}

fn black_rook_attacks(
    board: &BoardState,
    tables: &Tables,
    pinned_pieces: u64,
    king: u64,
    moves: &mut Vec<MoveRep>,
) {
    let black_occupancy = board.black_occupancy();
    let occupancy = board.occupancy();
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
            if attack.starting_square & pinned_pieces == 0 || board.pin_safe(tables, king, &attack)
            {
                moves.push(attack);
            }
        }
    }
}

fn black_bishop_attacks(
    board: &BoardState,
    tables: &Tables,
    pinned_pieces: u64,
    king: u64,
    moves: &mut Vec<MoveRep>,
) {
    let black_occupancy = board.black_occupancy();
    let occupancy = board.occupancy();
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
                attacked_type,
            };
            if attack.starting_square & pinned_pieces == 0 || board.pin_safe(tables, king, &attack)
            {
                moves.push(attack);
            }
        }
    }
}

fn black_queen_attacks(
    board: &BoardState,
    tables: &Tables,
    pinned_pieces: u64,
    king: u64,
    moves: &mut Vec<MoveRep>,
) {
    let black_occupancy = board.black_occupancy();
    let occupancy = board.occupancy();
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
            if attack.starting_square & pinned_pieces == 0 || board.pin_safe(tables, king, &attack)
            {
                moves.push(attack);
            }
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
                attacked_type,
            };
            if attack.starting_square & pinned_pieces == 0 || board.pin_safe(tables, king, &attack)
            {
                moves.push(attack);
            }
        }
    }
}

fn black_king_attacks(
    board: &BoardState,
    tables: &Tables,
    pinned_pieces: u64,
    king: u64,
    moves: &mut Vec<MoveRep>,
) {
    let black_occupancy = board.black_occupancy();
    let mut king_bb = board.black_king;
    while king_bb != 0 {
        let start_square = pop_lsb(&mut king_bb) as u64;
        let mut attacks = tables.king_attacks[start_square as usize] & !black_occupancy;
        while attacks != 0 {
            let end_square = pop_lsb(&mut attacks) as u64;
            let attacked_type = board.get_piece_type(1 << end_square);
            let white_attack_mask = board.white_attack_mask(tables);
            let attack = MoveRep {
                starting_square: 1 << start_square,
                ending_square: 1 << end_square,
                promotion: None,
                moved_type: PieceType::King,
                attacked_type,
            };
            if white_attack_mask & 1 << end_square == 0
                && (attack.starting_square & pinned_pieces == 0
                    || board.pin_safe(tables, king, &attack))
            {
                moves.push(attack);
            }
        }
    }
}

#[inline]
/// Get and remove the lsb as a square index
pub fn pop_lsb(bb: &mut u64) -> usize {
    let lsb = bb.trailing_zeros() as usize;
    *bb ^= 1 << lsb;
    lsb
}

#[cfg(test)]
mod tests {

    use crate::board::print_bitboard;

    use super::*;

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
    fn test_gen_blocking_moves_1() {
        let board = BoardState::state_from_string_fen(
            "rn1qkbnr/pppppppp/8/5b2/8/2NK4/P1PP1PPP/R1BQ1BNR w kq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let expected_move = MoveRep {
            starting_square: 1 << Tables::C3,
            ending_square: 1 << Tables::E4,
            promotion: None,
            moved_type: PieceType::Knight,
            attacked_type: None,
        };

        let results = generate_blocking_moves(&board, &tables, 1 << Tables::D3, 1 << Tables::F5);
        assert_eq!(results.len(), 1);
        assert!(results.contains(&expected_move));
    }

    #[test]
    fn test_gen_blocking_moves_2() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/p1pppppp/1p6/8/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let expected_move_1 = MoveRep::new(
            1 << Tables::C7,
            1 << Tables::C6,
            None,
            PieceType::Pawn,
            None,
        );

        let expected_move_2 = MoveRep::new(
            1 << Tables::D7,
            1 << Tables::D5,
            None,
            PieceType::Pawn,
            None,
        );

        let expected_move_3 = MoveRep::new(
            1 << Tables::B8,
            1 << Tables::C6,
            None,
            PieceType::Knight,
            None,
        );

        let expected_move_4 = MoveRep::new(
            1 << Tables::C8,
            1 << Tables::B7,
            None,
            PieceType::Bishop,
            None,
        );

        let results = generate_blocking_moves(&board, &tables, 1 << Tables::A8, 1 << Tables::G2);
        assert_eq!(results.len(), 4);
        assert!(results.contains(&expected_move_1));
        assert!(results.contains(&expected_move_2));
        assert!(results.contains(&expected_move_3));
        assert!(results.contains(&expected_move_4));
    }

    #[test]
    fn test_gen_blocking_moves_3() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppp1ppp/4p3/8/8/BP2P3/P1PP1PPP/RN1QKBNR b KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let expected_move_1 = MoveRep::new(
            1 << Tables::G8,
            1 << Tables::E7,
            None,
            PieceType::Knight,
            None,
        );

        let expected_move_2 = MoveRep::new(
            1 << Tables::D8,
            1 << Tables::E7,
            None,
            PieceType::Queen,
            None,
        );

        let expected_move_3 = MoveRep::new(
            1 << Tables::D7,
            1 << Tables::D6,
            None,
            PieceType::Pawn,
            None,
        );

        let expected_move_4 = MoveRep::new(
            1 << Tables::C7,
            1 << Tables::C5,
            None,
            PieceType::Pawn,
            None,
        );

        let results = generate_blocking_moves(&board, &tables, 1 << Tables::F8, 1 << Tables::A3);
        assert_eq!(results.len(), 4);
        assert!(results.contains(&expected_move_1));
        assert!(results.contains(&expected_move_2));
        assert!(results.contains(&expected_move_3));
        assert!(results.contains(&expected_move_4));
    }

    #[test]
    fn test_gen_blocking_moves_4() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/ppp1pppp/8/8/8/8/PPP1PPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );

        let tables = Tables::new();

        let expected_move_1 = MoveRep::new(
            1 << Tables::C8,
            1 << Tables::D7,
            None,
            PieceType::Bishop,
            None,
        );

        let expected_move_2 = MoveRep::new(
            1 << Tables::B8,
            1 << Tables::D7,
            None,
            PieceType::Knight,
            None,
        );

        let results = generate_blocking_moves(&board, &tables, 1 << Tables::D8, 1 << Tables::D1);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&expected_move_1));
        assert!(results.contains(&expected_move_2));
    }

    #[test]
    fn king_escape_1() {
        let board = BoardState::state_from_string_fen(
            "rnbq1bnr/pppp1ppp/8/4k2R/8/8/PPPPPPP1/RNBQKBN1 b Q - 0 1".to_string(),
        );
        let tables = Tables::new();

        let expected_move_1 = MoveRep::new(
            1 << Tables::E5,
            1 << Tables::F6,
            None,
            PieceType::King,
            None,
        );
        let expected_move_2 = MoveRep::new(
            1 << Tables::E5,
            1 << Tables::E6,
            None,
            PieceType::King,
            None,
        );
        let expected_move_3 = MoveRep::new(
            1 << Tables::E5,
            1 << Tables::D6,
            None,
            PieceType::King,
            None,
        );
        let expected_move_4 = MoveRep::new(
            1 << Tables::E5,
            1 << Tables::D4,
            None,
            PieceType::King,
            None,
        );
        let expected_move_5 = MoveRep::new(
            1 << Tables::E5,
            1 << Tables::E4,
            None,
            PieceType::King,
            None,
        );
        let expected_move_6 = MoveRep::new(
            1 << Tables::E5,
            1 << Tables::F4,
            None,
            PieceType::King,
            None,
        );

        let results = move_king_to_safety(&board, &tables);
        assert_eq!(results.len(), 6);
        assert!(results.contains(&expected_move_1));
        assert!(results.contains(&expected_move_2));
        assert!(results.contains(&expected_move_3));
        assert!(results.contains(&expected_move_4));
        assert!(results.contains(&expected_move_5));
        assert!(results.contains(&expected_move_6));
    }

    #[test]
    fn king_escape_2() {
        let board = BoardState::state_from_string_fen("8/8/8/8/4k3/2KP4/8/8 b - - 0 1".to_string());
        let tables = Tables::new();

        let expected_move_1 = MoveRep::new(
            1 << Tables::E4,
            1 << Tables::E3,
            None,
            PieceType::King,
            None,
        );
        let expected_move_2 = MoveRep::new(
            1 << Tables::E4,
            1 << Tables::F3,
            None,
            PieceType::King,
            None,
        );
        let expected_move_3 = MoveRep::new(
            1 << Tables::E4,
            1 << Tables::F4,
            None,
            PieceType::King,
            None,
        );
        let expected_move_4 = MoveRep::new(
            1 << Tables::E4,
            1 << Tables::F5,
            None,
            PieceType::King,
            None,
        );
        let expected_move_5 = MoveRep::new(
            1 << Tables::E4,
            1 << Tables::E5,
            None,
            PieceType::King,
            None,
        );
        let expected_move_6 = MoveRep::new(
            1 << Tables::E4,
            1 << Tables::D5,
            None,
            PieceType::King,
            None,
        );

        let results = move_king_to_safety(&board, &tables);
        assert_eq!(results.len(), 6);
        assert!(results.contains(&expected_move_1));
        assert!(results.contains(&expected_move_2));
        assert!(results.contains(&expected_move_3));
        assert!(results.contains(&expected_move_4));
        assert!(results.contains(&expected_move_5));
        assert!(results.contains(&expected_move_6));
    }

    #[test]
    fn king_escape_3() {
        let board =
            BoardState::state_from_string_fen("3Q1k2/4Q3/8/8/8/2K5/8/8 b - - 0 1".to_string());
        let tables = Tables::new();
        let results = move_king_to_safety(&board, &tables);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn king_escape_4() {
        let board = BoardState::state_from_string_fen(
            "rnb1kbnr/ppppqppp/8/8/8/8/PPP2PPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let expected_move = MoveRep::new(
            1 << Tables::E1,
            1 << Tables::D2,
            None,
            PieceType::King,
            None,
        );
        let results = move_king_to_safety(&board, &tables);
        for mv in &results {
            println!("{mv:?}");
        }
        assert_eq!(results.len(), 1);
        assert!(results.contains(&expected_move));
    }

    #[test]
    fn king_escape_5() {
        let board = BoardState::state_from_string_fen(
            "rnbq1bnr/ppppkppp/4pB2/8/8/1P6/P1PPPPPP/RN1QKBNR b - - 0 1".to_string(),
        );

        let tables = Tables::new();
        let results = generate(&board, &tables);
        for mv in &results {
            println!("{mv:?}");
        }
        assert_eq!(results.len(), 5);
    }

    #[test]
    fn test_king_in_check_1() {
        let board = BoardState::state_from_string_fen(
            "rn1qkbnr/2pppppp/bp6/p7/8/1P2P3/P1PPKPPP/RNBQ1BNR w - - 0 1".to_string(),
        );
        let tables = Tables::new();

        let unexpected_move = MoveRep::new(
            1 << Tables::B3,
            1 << Tables::B5,
            None,
            PieceType::Pawn,
            None,
        );

        let results = generate(&board, &tables);
        // for mv in &results {
        //     println!("{:?}", mv);
        // }
        assert!(!results.contains(&unexpected_move));
        // panic!();
    }

    #[test]
    fn test_king_in_check_2() {
        let board = BoardState::state_from_string_fen(
            "rnb1kbnr/pp1ppppp/8/q7/2p5/2KP4/PPP1PPPP/RNBQ1BNR w kq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let expected_move = MoveRep::new(
            1 << Tables::C3,
            1 << Tables::C4,
            None,
            PieceType::King,
            Some(PieceType::Pawn),
        );
        let moves = generate(&board, &tables);
        for mv in &moves {
            println!("{mv:?}");
        }
        assert!(moves.contains(&expected_move));
    }

    #[test]
    fn test_sample_bishop_move() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/1ppppp1p/8/p5p1/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 1".to_string(),
        );

        let tables = Tables::new();

        let expected_move = MoveRep::new(
            1 << Tables::B2,
            1 << Tables::H8,
            None,
            PieceType::Bishop,
            Some(PieceType::Rook),
        );

        let results = generate(&board, &tables);
        assert!(results.contains(&expected_move));
    }

    #[test]
    fn test_white_kingside_castle() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/1ppp2pp/4pp2/p7/2B5/4P3/PPPPNPPP/RNBQK2R w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let expected_move = MoveRep::new(
            1 << Tables::E1,
            1 << Tables::G1,
            Some(Promotion::Castle),
            PieceType::King,
            None,
        );
        let moves = generate(&board, &tables);
        assert!(moves.contains(&expected_move));
    }

    #[test]
    fn test_white_queenside_castle() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/1ppp2pp/4pp2/p7/8/BPNP4/P1PQPPPP/R3KBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let expected_move = MoveRep::new(
            1 << Tables::E1,
            1 << Tables::C1,
            Some(Promotion::Castle),
            PieceType::King,
            None,
        );
        let moves = generate(&board, &tables);
        assert!(moves.contains(&expected_move));
    }

    #[test]
    fn test_black_kingside_castle() {
        let board = BoardState::state_from_string_fen(
            "rnbqk2r/1ppp2pp/3bpp1n/p7/8/BPNP4/P1PQPPPP/R3KBNR b KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let expected_move = MoveRep::new(
            1 << Tables::E8,
            1 << Tables::G8,
            Some(Promotion::Castle),
            PieceType::King,
            None,
        );
        let moves = generate(&board, &tables);
        assert!(moves.contains(&expected_move));
    }

    #[test]
    fn test_black_queenside_castle() {
        let board = BoardState::state_from_string_fen(
            "r3kbnr/pppqpppp/2npb3/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let expected_move = MoveRep::new(
            1 << Tables::E8,
            1 << Tables::C8,
            Some(Promotion::Castle),
            PieceType::King,
            None,
        );
        let moves = generate(&board, &tables);
        assert!(moves.contains(&expected_move));
    }

    #[test]
    fn test_castle_blocked_1() {
        let board = BoardState::state_from_string_fen(
            "r3kbnr/pp1qpppp/1Bnpb3/2p5/8/8/PPPPPPPP/RN1QKBNR b KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let unexpected_move = MoveRep::new(
            1 << Tables::E8,
            1 << Tables::C8,
            Some(Promotion::Castle),
            PieceType::King,
            None,
        );

        let moves = generate(&board, &tables);
        assert!(!moves.contains(&unexpected_move));
    }

    #[test]
    fn test_castle_blocked_2() {
        let board = BoardState::state_from_string_fen(
            "r3kbnr/pp1Bpppp/2npb3/2pq4/8/8/PPPPPPPP/RN1QKBNR b KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let unexpected_move = MoveRep::new(
            1 << Tables::E8,
            1 << Tables::C8,
            Some(Promotion::Castle),
            PieceType::King,
            None,
        );

        let moves = generate(&board, &tables);
        assert!(!moves.contains(&unexpected_move));
    }

    #[test]
    fn test_castle_blocked_3() {
        let board = BoardState::state_from_string_fen(
            "r3kbnr/pp2pppp/2npb3/2pq4/8/8/PPPPPPPP/RN1QKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let unexpected_move = MoveRep::new(
            1 << Tables::E1,
            1 << Tables::C1,
            Some(Promotion::Castle),
            PieceType::King,
            None,
        );

        let moves = generate(&board, &tables);
        assert!(!moves.contains(&unexpected_move));
    }

    #[test]
    fn test_castle_blocked_4() {
        let board = BoardState::state_from_string_fen(
            "r3kbnr/pp2pppp/2npb3/2p5/3q1BP1/5PN1/PPPPP2P/RN1QK2R w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let unexpected_move = MoveRep::new(
            1 << Tables::E1,
            1 << Tables::G1,
            Some(Promotion::Castle),
            PieceType::King,
            None,
        );

        let moves = generate(&board, &tables);
        assert!(!moves.contains(&unexpected_move));
    }

    #[test]
    fn test_castle_blocked_5() {
        let board = BoardState::state_from_string_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q2/PPPBBPpP/1R2K2R w Kkq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let unexpected_mov = MoveRep::new(
            1 << Tables::E1,
            1 << Tables::G1,
            Some(Promotion::Castle),
            PieceType::King,
            None,
        );
        let results = generate(&board, &tables);
        print_bitboard(board.white_king >> 1);
        print_bitboard(board.white_king >> 2);
        print_bitboard(board.black_attacking(&tables, board.white_king >> 1));
        print_bitboard(board.black_attacking(&tables, board.white_king >> 2));
        assert!(!results.contains(&unexpected_mov));
    }
}
