#[derive(Debug)]
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

    pub fn apply_move(&mut self, chess_move: &str) -> Result<(), String> {
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

        // Clear the starting posisition, and set then ending posistion
        match start_mask {
            // White

            // Pawns
            e if e & self.white_pawns != 0 => {
                self.white_pawns &= !e;
                self.white_pawns |= end_mask;
            }

            // Knights
            e if e & self.white_knights != 0 => {
                self.white_knights &= !e;
                self.white_knights |= end_mask;
            }

            // Biships
            e if e & self.white_bishops != 0 => {
                self.white_bishops &= !e;
                self.white_bishops |= end_mask;
            }

            // Rooks
            e if e & self.white_rooks != 0 => {
                self.white_rooks &= !e;
                self.white_rooks |= end_mask;
            }

            // Queens
            e if e & self.white_queens != 0 => {
                self.white_queens &= !e;
                self.white_queens |= end_mask;
            }

            // Kings
            e if e & self.white_king != 0 => {
                self.white_king &= !e;
                self.white_king |= end_mask;
            }

            // Black

            // Pawns
            e if e & self.black_pawns != 0 => {
                self.black_pawns &= !e;
                self.black_pawns |= end_mask;
            }

            // Knights
            e if e & self.black_knights != 0 => {
                self.black_knights &= !e;
                self.black_knights |= end_mask;
            }

            // Biships
            e if e & self.black_bishops != 0 => {
                self.black_bishops &= !e;
                self.black_bishops |= end_mask;
            }

            // Rooks
            e if e & self.black_rooks != 0 => {
                self.black_rooks &= !e;
                self.black_rooks |= end_mask;
            }

            // Queens
            e if e & self.black_queens != 0 => {
                self.black_queens &= !e;
                self.black_queens |= end_mask;
            }

            // Kings
            e if e & self.black_king != 0 => {
                self.black_king &= !e;
                self.black_king |= end_mask;
            }

            _ => return Err(format!("No piece for move {}", chess_move)),
        }

        // Before returning, change the side to move
        self.white_to_move = !self.white_to_move;

        Ok(())
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
