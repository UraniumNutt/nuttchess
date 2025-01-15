#[derive(Debug, Copy, Clone)]
pub struct BoardState {
    white_pawns: u64,
    white_knights: u64,
    white_rooks: u64,
    white_bishops: u64,
    white_queens: u64,
    white_king: u64,
    white_queenside_castle_rights: bool,
    white_kingside_castle_rights: bool,

    black_pawns: u64,
    black_knights: u64,
    black_rooks: u64,
    black_bishops: u64,
    black_queens: u64,
    black_king: u64,
    black_queenside_castle_rights: bool,
    black_kingside_castle_rights: bool,

    white_to_move: bool,
    en_passant_target: u64,
    reversable_move_counter: u8,
    fullmove_counter: u16,
}

#[derive(Copy, Clone)]
pub struct MoveRep {
    starting_square: u64,
    ending_square: u64,
    promotion: Option<Promotion>,
}

#[derive(Copy, Clone)]
enum Promotion {
    Queen,
    Bishop,
    Rook,
    Knight,
}

pub struct TreeNode {
    pub board_state: BoardState,
    pub applied_move: Option<MoveRep>,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn get_leaf_nodes(&self) -> u64 {
        if self.children.len() == 0 {
            return 1;
        }
        let mut count = 0;
        for child in &self.children {
            count += child.get_leaf_nodes();
        }
        count
    }
}

impl BoardState {
    pub fn starting_state() -> BoardState {
        BoardState {
            white_pawns: 0xff00,
            white_knights: 0x42,
            white_rooks: 0x81,
            white_bishops: 0x24,
            white_queens: 0x10,
            white_king: 0x8,
            white_queenside_castle_rights: true,
            white_kingside_castle_rights: true,

            black_pawns: 0xff000000000000,
            black_knights: 0x4200000000000000,
            black_rooks: 0x8100000000000000,
            black_bishops: 0x2400000000000000,
            black_queens: 0x1000000000000000,
            black_king: 0x800000000000000,
            black_queenside_castle_rights: true,
            black_kingside_castle_rights: true,

            white_to_move: true,
            en_passant_target: 0x0,
            reversable_move_counter: 0,
            fullmove_counter: 0,
        }
    }

    fn empty_state() -> BoardState {
        BoardState {
            white_pawns: 0,
            white_knights: 0,
            white_rooks: 0,
            white_bishops: 0,
            white_queens: 0,
            white_king: 0,
            white_queenside_castle_rights: false,
            white_kingside_castle_rights: false,

            black_pawns: 0,
            black_knights: 0,
            black_rooks: 0,
            black_bishops: 0,
            black_queens: 0,
            black_king: 0,
            black_queenside_castle_rights: false,
            black_kingside_castle_rights: false,

            white_to_move: true,
            en_passant_target: 0,
            reversable_move_counter: 0,
            fullmove_counter: 0,
        }
    }

    fn white_occupancy(&self) -> u64 {
        self.white_pawns
            | self.white_knights
            | self.white_rooks
            | self.white_bishops
            | self.white_queens
            | self.white_king
    }

    fn black_occupancy(&self) -> u64 {
        self.black_pawns
            | self.black_knights
            | self.black_rooks
            | self.black_bishops
            | self.black_queens
            | self.black_king
    }

    pub fn state_from_fen<'a>(
        mut fen_tokens: impl Iterator<Item = &'a str>,
    ) -> Result<BoardState, String> {
        // Split the fen string at every / and space
        // let mut fen_tokens = fen.split(|c| c == '/' || c == ' ');

        let mut state = BoardState::empty_state();
        let mut shift_value: u64 = 1 << 63;

        // Parse the placement data

        if let Some(token) = fen_tokens.next() {
            for character in token.chars() {
                if character == '/' {
                    continue;
                }
                // If the character is a digit, shift over the mask by that amount
                if let Some(digit) = character.to_digit(10) {
                    shift_value >>= digit;
                } else {
                    // If the character is not a digit, match it to the piece
                    // type and set the relevent bit in the board state
                    match character {
                        // First match  black pieces (lowercase)
                        // p -> pawn
                        // r -> rook
                        // n -> knight
                        // b -> bishop
                        // q -> queen
                        // k -> king
                        'p' => state.black_pawns |= shift_value,
                        'r' => state.black_rooks |= shift_value,
                        'n' => state.black_knights |= shift_value,
                        'b' => state.black_bishops |= shift_value,
                        'q' => state.black_queens |= shift_value,
                        'k' => state.black_king |= shift_value,

                        // Now try the white pieces (uppercase)
                        'P' => state.white_pawns |= shift_value,
                        'R' => state.white_rooks |= shift_value,
                        'N' => state.white_knights |= shift_value,
                        'B' => state.white_bishops |= shift_value,
                        'Q' => state.white_queens |= shift_value,
                        'K' => state.white_king |= shift_value,

                        _ => return Err(format!("Unexpected character found in {}", character)),
                    }
                    shift_value >>= 1;
                }
                // shift_value >>= 1;
            }
        } else {
            return Err("No fenstring placement data found".to_string());
        }
        // Check that the proper number of positions were fed in
        if shift_value != 0 {
            return Err(format!(
                "Incorect number of positions found (shift_value is {})",
                shift_value
            ));
        }

        // Now parse the active color
        if let Some(side_to_move) = fen_tokens.next() {
            match side_to_move {
                "w" => state.white_to_move = true,
                "b" => state.white_to_move = false,
                _ => return Err(format!("Invalid side to move \"{}\"", side_to_move)),
            }
        } else {
            return Err("String does not have enough tokens to be a valid fen string".to_string());
        }

        // castling rights
        if let Some(castle_rights) = fen_tokens.next() {
            for character in castle_rights.chars() {
                match character {
                    // K -> white kingside
                    // Q -> white queen side
                    // k -> black kingside
                    // q -> black queenside
                    'K' => state.white_kingside_castle_rights = true,
                    'Q' => state.white_queenside_castle_rights = true,
                    'k' => state.black_kingside_castle_rights = true,
                    'q' => state.black_queenside_castle_rights = true,
                    // No castle rights
                    '-' => continue,
                    _ => {
                        return Err(format!(
                            "Unknown character \"{}\" found in castle rights field",
                            character
                        ))
                    }
                }
            }
        } else {
            return Err("String does not have enough tokens to be a valid fen string".to_string());
        }

        // En passant target
        if let Some(en_passant_target) = fen_tokens.next() {
            if en_passant_target == "-" {
                // No en passant target
                state.en_passant_target = 0;
            } else {
                let mut en_passant_shift = 1;
                let mut target_chars = en_passant_target.chars();
                if let (Some(file), Some(rank)) = (target_chars.next(), target_chars.next()) {
                    // Match the rank and file to get the correct mask
                    let file_shift = match file {
                        'h' => 0,
                        'g' => 1,
                        'f' => 2,
                        'e' => 3,
                        'd' => 4,
                        'c' => 5,
                        'b' => 6,
                        'a' => 7,
                        _ => return Err(format!("Unrecognized value \"{}\" found in file", file)),
                    };
                    let rank_shift: i32;
                    if let Some(rank_value) = rank.to_digit(10) {
                        rank_shift = rank_value as i32;
                    } else {
                        return Err(format!("Unrecognized value \"{}\" found in rank", rank));
                    }
                    en_passant_shift = (1 << file_shift) << ((rank_shift - 1) * 8);
                    state.en_passant_target = en_passant_shift;
                } else {
                    return Err(
                        "En passant target does not have the expected number of characters"
                            .to_string(),
                    );
                }
            }
        } else {
            return Err("String does not have enough tokens to be a valid fen string".to_string());
        }

        // Parse the halfmove clock
        if let Some(half_move_clock) = fen_tokens.next() {
            if let Ok(half_move_int) = half_move_clock.parse::<u8>() {
                state.reversable_move_counter = half_move_int;
            } else {
                return Err("Error parsing the number of halfmoves".to_string());
            }
        } else {
            return Err("String does not have enough tokens to be a valid fen string".to_string());
        }

        // Parse the fullmove counter
        if let Some(full_move_clock) = fen_tokens.next() {
            if let Ok(full_move_clock_int) = full_move_clock.parse::<u16>() {
                state.fullmove_counter = full_move_clock_int;
            } else {
                return Err("Error parsing the number of fullmoves".to_string());
            }
        } else {
            return Err("String does not have enough tokens to be a valid fen string".to_string());
        }

        Ok(state)
    }

    pub fn apply_move(&self, play: &MoveRep) -> Result<BoardState, String> {
        // Clear the starting posisition, and set then ending posistion
        let (start_mask, end_mask, promotion) =
            (&play.starting_square, &play.ending_square, &play.promotion);
        let mut state = self.to_owned();
        match start_mask {
            // White

            // Pawns
            e if e & state.white_pawns != 0 => {
                state.white_pawns &= !e;
                state.white_pawns |= end_mask;
            }

            // Knights
            e if e & state.white_knights != 0 => {
                state.white_knights &= !e;
                state.white_knights |= end_mask;
            }

            // Biships
            e if e & state.white_bishops != 0 => {
                state.white_bishops &= !e;
                state.white_bishops |= end_mask;
            }

            // Rooks
            e if e & state.white_rooks != 0 => {
                state.white_rooks &= !e;
                state.white_rooks |= end_mask;
            }

            // Queens
            e if e & state.white_queens != 0 => {
                state.white_queens &= !e;
                state.white_queens |= end_mask;
            }

            // Kings
            e if e & state.white_king != 0 => {
                state.white_king &= !e;
                state.white_king |= end_mask;
            }

            // Black

            // Pawns
            e if e & state.black_pawns != 0 => {
                state.black_pawns &= !e;
                state.black_pawns |= end_mask;
            }

            // Knights
            e if e & state.black_knights != 0 => {
                state.black_knights &= !e;
                state.black_knights |= end_mask;
            }

            // Biships
            e if e & state.black_bishops != 0 => {
                state.black_bishops &= !e;
                state.black_bishops |= end_mask;
            }

            // Rooks
            e if e & state.black_rooks != 0 => {
                state.black_rooks &= !e;
                state.black_rooks |= end_mask;
            }

            // Queens
            e if e & state.black_queens != 0 => {
                state.black_queens &= !e;
                state.black_queens |= end_mask;
            }

            // Kings
            e if e & state.black_king != 0 => {
                state.black_king &= !e;
                state.black_king |= end_mask;
            }

            // TODO implement some formating so we can print the play
            // _ => return Err(format!("No piece for move {}", play)),
            _ => return Err("No target piece found".to_string()),
        }

        // Before returning, change the side to move
        state.white_to_move = !state.white_to_move;
        Ok(state)
    }

    pub fn search(self, depth: u64) -> Option<TreeNode> {
        if depth == 0 {
            return None;
        }

        let mut children: Vec<TreeNode> = vec![];
        let child_moves = self.generate_moves();
        for child_move in child_moves {
            let child_node = self.apply_move(&child_move).unwrap().search(depth - 1);
            let node = TreeNode {
                board_state: self.apply_move(&child_move).unwrap(),
                applied_move: Some(child_move),
                children: match child_node {
                    Some(e) => e.children,
                    None => vec![],
                },
            };
            children.push(node);
        }
        let tree = TreeNode {
            board_state: self,
            applied_move: None,
            children: children,
        };
        Some(tree)
    }

    // generate the moves from a given position
    pub fn generate_moves(&self) -> Vec<MoveRep> {
        let mut moves = Vec::new();

        // Generate pawn moves
        match self.white_to_move {
            // white pawns
            true => moves.append(&mut self.white_pawn_moves()),
            // black pawns
            false => moves.append(&mut self.black_pawn_moves()),
        }

        // Knight moves
        moves.append(&mut self.knight_moves());

        moves
    }

    fn knight_moves(&self) -> Vec<MoveRep> {
        let mut moves = Vec::new();

        for shift_value in 0..64 {
            let mask = 1 << shift_value;
            if self.white_to_move {
                if mask & self.white_knights != 0 {
                    let start = mask;
                    // case '1'
                    if !rank_h(mask) && !(file_7(mask) || file_8(mask)) {
                        let end = mask << 15;
                        if end & self.white_occupancy() == 0 {
                            moves.push(MoveRep {
                                starting_square: start,
                                ending_square: end,
                                promotion: None,
                            })
                        }
                    }
                    // case '2'
                    if !(rank_h(mask) || rank_g(mask)) && !file_8(mask) {
                        let end = mask << 6;
                        if end & self.white_occupancy() == 0 {
                            moves.push(MoveRep {
                                starting_square: start,
                                ending_square: end,
                                promotion: None,
                            })
                        }
                    }
                    // case '3'
                    if !(rank_h(mask) || rank_g(mask)) && !file_1(mask) {
                        let end = mask >> 10;
                        if end & self.white_occupancy() == 0 {
                            moves.push(MoveRep {
                                starting_square: start,
                                ending_square: end,
                                promotion: None,
                            })
                        }
                    }
                    // case '4'
                    if !rank_h(mask) && !(file_1(mask) || file_2(mask)) {
                        let end = mask >> 17;
                        if end & self.white_occupancy() == 0 {
                            moves.push(MoveRep {
                                starting_square: start,
                                ending_square: end,
                                promotion: None,
                            })
                        }
                    }
                    // case '5'
                    if !rank_a(mask) && !(file_1(mask) || file_2(mask)) {
                        let end = mask >> 15;
                        if end & self.white_occupancy() == 0 {
                            moves.push(MoveRep {
                                starting_square: start,
                                ending_square: end,
                                promotion: None,
                            })
                        }
                    }
                    // case '6'
                    if !(rank_a(mask) && rank_b(mask)) && !file_1(mask) {
                        let end = mask >> 6;
                        if end & self.white_occupancy() == 0 {
                            moves.push(MoveRep {
                                starting_square: start,
                                ending_square: end,
                                promotion: None,
                            })
                        }
                    }
                    // case '7'
                    if !(rank_a(mask) || rank_b(mask)) && !file_8(mask) {
                        let end = mask << 10;
                        if end & self.white_occupancy() == 0 {
                            moves.push(MoveRep {
                                starting_square: start,
                                ending_square: end,
                                promotion: None,
                            })
                        }
                    }
                    // case '8'
                    if !rank_a(mask) && !(file_7(mask) || file_8(mask)) {
                        let end = mask << 17;
                        if end & self.white_occupancy() == 0 {
                            moves.push(MoveRep {
                                starting_square: start,
                                ending_square: end,
                                promotion: None,
                            })
                        }
                    }
                }
            } else {
                if mask & self.black_knights != 0 {
                    let start = mask;
                    // case '1'
                    if !rank_h(mask) && !(file_7(mask) || file_8(mask)) {
                        let end = mask << 15;
                        if end & self.black_occupancy() == 0 {
                            moves.push(MoveRep {
                                starting_square: start,
                                ending_square: end,
                                promotion: None,
                            })
                        }
                    }
                    // case '2'
                    if !(rank_h(mask) || rank_g(mask)) && !file_8(mask) {
                        let end = mask << 6;
                        if end & self.black_occupancy() == 0 {
                            moves.push(MoveRep {
                                starting_square: start,
                                ending_square: end,
                                promotion: None,
                            })
                        }
                    }
                    // case '3'
                    if !(rank_h(mask) || rank_g(mask)) && !file_1(mask) {
                        let end = mask >> 10;
                        if end & self.black_occupancy() == 0 {
                            moves.push(MoveRep {
                                starting_square: start,
                                ending_square: end,
                                promotion: None,
                            })
                        }
                    }
                    // case '4'
                    if !rank_h(mask) && !(file_1(mask) || file_2(mask)) {
                        let end = mask >> 17;
                        if end & self.black_occupancy() == 0 {
                            moves.push(MoveRep {
                                starting_square: start,
                                ending_square: end,
                                promotion: None,
                            })
                        }
                    }
                    // case '5'
                    if !rank_a(mask) && !(file_1(mask) || file_2(mask)) {
                        let end = mask >> 15;
                        if end & self.black_occupancy() == 0 {
                            moves.push(MoveRep {
                                starting_square: start,
                                ending_square: end,
                                promotion: None,
                            })
                        }
                    }
                    // case '6'
                    if !(rank_a(mask) && rank_b(mask)) && !file_1(mask) {
                        let end = mask >> 6;
                        if end & self.black_occupancy() == 0 {
                            moves.push(MoveRep {
                                starting_square: start,
                                ending_square: end,
                                promotion: None,
                            })
                        }
                    }
                    // case '7'
                    if !(rank_a(mask) || rank_b(mask)) && !file_8(mask) {
                        let end = mask << 10;
                        if end & self.black_occupancy() == 0 {
                            moves.push(MoveRep {
                                starting_square: start,
                                ending_square: end,
                                promotion: None,
                            })
                        }
                    }
                    // case '8'
                    if !rank_a(mask) && !(file_7(mask) || file_8(mask)) {
                        let end = mask << 17;
                        if end & self.black_occupancy() == 0 {
                            moves.push(MoveRep {
                                starting_square: start,
                                ending_square: end,
                                promotion: None,
                            })
                        }
                    }
                }
            }
        }
        moves
    }

    fn black_pawn_moves(&self) -> Vec<MoveRep> {
        let mut moves = Vec::new();

        for shift_value in 0..64 {
            let mask = 1 << shift_value;

            // If there is no pawn there, go skip
            if mask & self.black_pawns == 0 {
                continue;
            }

            // Single push
            let start = mask;
            let end = mask >> 8;
            moves.push(MoveRep {
                starting_square: start,
                ending_square: end,
                promotion: None,
            });
            // Double push
            let start = mask;
            let end = mask >> 16;
            moves.push(MoveRep {
                starting_square: start,
                ending_square: end,
                promotion: None,
            });
        }
        moves
    }

    fn white_pawn_moves(&self) -> Vec<MoveRep> {
        let mut moves = Vec::new();

        for shift_value in 0..64 {
            let mask = 1 << shift_value;

            // If there is no pawn there, go skip
            if mask & self.white_pawns == 0 {
                continue;
            }

            // Single push
            let start = mask;
            let end = mask << 8;
            moves.push(MoveRep {
                starting_square: start,
                ending_square: end,
                promotion: None,
            });
            // Double push
            let start = mask;
            let end = mask << 16;
            moves.push(MoveRep {
                starting_square: start,
                ending_square: end,
                promotion: None,
            });
        }
        moves
    }

    pub fn apply_string_move(&self, chess_move: &str) -> Result<BoardState, String> {
        // Get a bitboard mask of the starting move
        let start_mask = position_to_mask(
            chess_move.chars().nth(0).unwrap(),
            chess_move.chars().nth(1).unwrap(),
        )
        .unwrap();
        // Get a bitboard mask of the ending move
        let end_mask = position_to_mask(
            chess_move.chars().nth(2).unwrap(),
            chess_move.chars().nth(3).unwrap(),
        )
        .unwrap();

        let play = MoveRep {
            starting_square: start_mask,
            ending_square: end_mask,
            promotion: None,
        };

        self.apply_move(&play)
    }

    pub fn print_board(&self) {
        println!("{:#018x}: white pawns", self.white_pawns);
        println!("{:#018x}: white knights", self.white_knights);
        println!("{:#018x}: white bishops", self.white_bishops);
        println!("{:#018x}: white rooks", self.white_rooks);
        println!("{:#018x}: white queens", self.white_queens);
        println!("{:#018x}: white king", self.white_king);

        println!("{:#018x}: black pawns", self.black_pawns);
        println!("{:#018x}: black knights", self.black_knights);
        println!("{:#018x}: black bishops", self.black_bishops);
        println!("{:#018x}: black rooks", self.black_rooks);
        println!("{:#018x}: black queens", self.black_queens);
        println!("{:#018x}: black king", self.black_king);

        println!("{:#018x}: en passant target", self.en_passant_target);
    }
}

impl MoveRep {
    pub fn to_string(&self) -> Result<String, String> {
        let start = self.starting_square;
        let end = self.ending_square;
        let mut mov = String::new();
        mov.push_str(MoveRep::mask_to_string(start).unwrap().as_ref());
        mov.push_str(MoveRep::mask_to_string(end).unwrap().as_ref());
        Ok(mov)
    }

    fn mask_to_string(mask: u64) -> Result<String, String> {
        let mut pos = String::new();

        let file = mask.ilog2() / 8;
        let rank = mask.ilog2() % 8;
        match rank {
            0 => pos.push('h'),
            1 => pos.push('g'),
            2 => pos.push('f'),
            3 => pos.push('e'),
            4 => pos.push('d'),
            5 => pos.push('c'),
            6 => pos.push('b'),
            7 => pos.push('a'),
            _ => return Err("Invalid mask found".to_string()),
        }
        match file {
            0 => pos.push('1'),
            1 => pos.push('2'),
            2 => pos.push('3'),
            3 => pos.push('4'),
            4 => pos.push('5'),
            5 => pos.push('6'),
            6 => pos.push('7'),
            7 => pos.push('8'),
            _ => return Err("Invalid mask found".to_string()),
        }

        Ok(pos)
    }
}

fn rank_a(mask: u64) -> bool {
    return mask.ilog2() % 8 == 7;
}
fn rank_b(mask: u64) -> bool {
    return mask.ilog2() % 8 == 6;
}
fn rank_c(mask: u64) -> bool {
    return mask.ilog2() % 8 == 5;
}
fn rank_d(mask: u64) -> bool {
    return mask.ilog2() % 8 == 4;
}
fn rank_e(mask: u64) -> bool {
    return mask.ilog2() % 8 == 3;
}
fn rank_f(mask: u64) -> bool {
    return mask.ilog2() % 8 == 2;
}
fn rank_g(mask: u64) -> bool {
    return mask.ilog2() % 8 == 1;
}
fn rank_h(mask: u64) -> bool {
    return mask.ilog2() % 8 == 0;
}

fn file_1(mask: u64) -> bool {
    return mask.ilog2() / 8 == 0;
}
fn file_2(mask: u64) -> bool {
    return mask.ilog2() / 8 == 1;
}
fn file_3(mask: u64) -> bool {
    return mask.ilog2() / 8 == 2;
}
fn file_4(mask: u64) -> bool {
    return mask.ilog2() / 8 == 3;
}
fn file_5(mask: u64) -> bool {
    return mask.ilog2() / 8 == 4;
}
fn file_6(mask: u64) -> bool {
    return mask.ilog2() / 8 == 5;
}
fn file_7(mask: u64) -> bool {
    return mask.ilog2() / 8 == 6;
}
fn file_8(mask: u64) -> bool {
    return mask.ilog2() / 8 == 7;
}

fn position_to_mask(file: char, rank: char) -> Result<u64, String> {
    let file_shift = match file {
        'h' => 0,
        'g' => 1,
        'f' => 2,
        'e' => 3,
        'd' => 4,
        'c' => 5,
        'b' => 6,
        'a' => 7,
        _ => return Err(format!("Unrecognized value \"{}\" found in file", file)),
    };
    let rank_shift: i32;
    if let Some(rank_value) = rank.to_digit(10) {
        rank_shift = rank_value as i32;
    } else {
        return Err(format!("Unrecognized value \"{}\" found in rank", rank));
    }
    Ok((1 << file_shift) << ((rank_shift - 1) * 8))
}
