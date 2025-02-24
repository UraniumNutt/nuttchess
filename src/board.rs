// use crate::Tables;
use crate::{generate::*, tables::Tables};
use std::io::{self, Write};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoardState {
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_rooks: u64,
    pub white_bishops: u64,
    pub white_queens: u64,
    pub white_king: u64,
    pub white_queenside_castle_rights: bool,
    pub white_kingside_castle_rights: bool,

    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_rooks: u64,
    pub black_bishops: u64,
    pub black_queens: u64,
    pub black_king: u64,
    pub black_queenside_castle_rights: bool,
    pub black_kingside_castle_rights: bool,

    pub white_to_move: bool,
    pub en_passant_target: u64,
    pub reversable_move_counter: u8,
    pub full_move_counter: u16,
    pub move_stack: Vec<MoveStackFrame>,
    pub move_stack_pointer: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MoveRep {
    pub starting_square: u64,
    pub ending_square: u64,
    pub promotion: Option<Promotion>,
    pub moved_type: PieceType,
    pub attacked_type: Option<PieceType>,
}

impl MoveRep {
    pub fn new(
        starting_square: u64,
        ending_square: u64,
        promotion: Option<Promotion>,
        piece_hint: PieceType,
        attacked_type: Option<PieceType>,
    ) -> MoveRep {
        MoveRep {
            starting_square,
            ending_square,
            promotion,
            moved_type: piece_hint,
            attacked_type,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Promotion {
    Queen,
    Bishop,
    Rook,
    Knight,
    Castle,
}

// Helps the move maker know what bitboard to manipulate
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

// Stores state of the board which can not be recovered when unmaking a move
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MoveStackFrame {
    en_passant_target: u64,
    reversable_move_counter: u8,
    fullmove_counter: u16,
    white_queenside_castle_rights: bool,
    white_kingside_castle_rights: bool,
    black_queenside_castle_rights: bool,
    black_kingside_castle_rights: bool,
}

impl MoveStackFrame {
    fn new() -> MoveStackFrame {
        MoveStackFrame {
            en_passant_target: 0,
            reversable_move_counter: 0,
            fullmove_counter: 0,
            white_queenside_castle_rights: true,
            white_kingside_castle_rights: true,
            black_queenside_castle_rights: true,
            black_kingside_castle_rights: true,
        }
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
            full_move_counter: 1,
            move_stack: vec![MoveStackFrame::new(); 0],
            move_stack_pointer: 0,
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
            full_move_counter: 0,
            move_stack: vec![MoveStackFrame::new(); 0],
            move_stack_pointer: 0,
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
                state.full_move_counter = full_move_clock_int;
            } else {
                return Err("Error parsing the number of fullmoves".to_string());
            }
        } else {
            return Err("String does not have enough tokens to be a valid fen string".to_string());
        }

        Ok(state)
    }

    pub fn state_from_string_fen(fen_string: String) -> BoardState {
        let tokens = fen_string.split(" ");
        BoardState::state_from_fen(tokens).unwrap()
    }

    fn move_rep_from_masks(&self, start: u64, end: u64) -> MoveRep {
        let moved_piece = self.get_piece_type(start);
        let attacked_piece = self.get_piece_type(end);
        MoveRep {
            starting_square: start,
            ending_square: end,
            promotion: None,
            moved_type: moved_piece.unwrap(),
            attacked_type: attacked_piece,
        }
    }

    pub fn apply_string_move(&mut self, play: String) {
        let char1 = play.chars().nth(0).unwrap();
        let char2 = play.chars().nth(1).unwrap();
        let char3 = play.chars().nth(2).unwrap();
        let char4 = play.chars().nth(3).unwrap();
        let char5 = play.chars().nth(4);

        let start = position_to_mask(char1, char2).unwrap();
        let end = position_to_mask(char3, char4).unwrap();
        let mut move_rep = self.move_rep_from_masks(start, end);
        if let Some(promotion) = char5 {
            match promotion {
                'q' => move_rep.promotion = Some(Promotion::Queen),
                'r' => move_rep.promotion = Some(Promotion::Rook),
                'b' => move_rep.promotion = Some(Promotion::Bishop),
                'n' => move_rep.promotion = Some(Promotion::Knight),
                _ => {}
            }
        }
        self.make(&move_rep);
    }

    /// Pushes the current nonreversible state to the stack
    fn push_state(&mut self) {
        let mut frame = MoveStackFrame::new();
        frame.en_passant_target = self.en_passant_target;
        frame.reversable_move_counter = self.reversable_move_counter;
        frame.fullmove_counter = self.full_move_counter;
        frame.white_queenside_castle_rights = self.white_queenside_castle_rights;
        frame.white_kingside_castle_rights = self.white_kingside_castle_rights;
        frame.black_queenside_castle_rights = self.black_queenside_castle_rights;
        frame.black_kingside_castle_rights = self.black_kingside_castle_rights;
        self.move_stack.push(frame);
        self.move_stack_pointer += 1;
    }

    /// Pops the nonreversible state from the stack
    fn pop_state(&mut self) {
        self.move_stack_pointer -= 1;
        let frame = self.move_stack.pop().unwrap();
        self.en_passant_target = frame.en_passant_target;
        self.reversable_move_counter = frame.reversable_move_counter;
        self.full_move_counter = frame.fullmove_counter;
        self.white_queenside_castle_rights = frame.white_queenside_castle_rights;
        self.white_kingside_castle_rights = frame.white_kingside_castle_rights;
        self.black_queenside_castle_rights = frame.black_queenside_castle_rights;
        self.black_kingside_castle_rights = frame.black_kingside_castle_rights;
    }

    // Changes the board state to reflect the move. Also pushes to the move stack
    pub fn make(&mut self, play: &MoveRep) {
        // If the move is castling, do the move logic here, and return (dont do the normal path)
        if play.promotion == Some(Promotion::Castle) {
            self.push_state();
            self.en_passant_target = 0;
            match play.ending_square {
                e if e == 1 << Tables::G1 => {
                    // White kingside
                    self.clear(play.starting_square, Some(PieceType::King));
                    self.clear(1 << Tables::H1, Some(PieceType::Rook));
                    self.set(play.ending_square, Some(PieceType::King));
                    self.set(1 << Tables::F1, Some(PieceType::Rook));
                    self.white_queenside_castle_rights = false;
                    self.white_kingside_castle_rights = false;
                }
                e if e == 1 << Tables::C1 => {
                    // White queenside
                    self.clear(play.starting_square, Some(PieceType::King));
                    self.clear(1 << Tables::A1, Some(PieceType::Rook));
                    self.set(play.ending_square, Some(PieceType::King));
                    self.set(1 << Tables::D1, Some(PieceType::Rook));
                    self.white_queenside_castle_rights = false;
                    self.white_kingside_castle_rights = false;
                }
                e if e == 1 << Tables::G8 => {
                    // Black kingside
                    self.clear(play.starting_square, Some(PieceType::King));
                    self.clear(1 << Tables::H8, Some(PieceType::Rook));
                    self.set(play.ending_square, Some(PieceType::King));
                    self.set(1 << Tables::F8, Some(PieceType::Rook));
                    self.black_queenside_castle_rights = false;
                    self.black_kingside_castle_rights = false;
                }
                e if e == 1 << Tables::C8 => {
                    // Black queenside
                    self.clear(play.starting_square, Some(PieceType::King));
                    self.clear(1 << Tables::A8, Some(PieceType::Rook));
                    self.set(play.ending_square, Some(PieceType::King));
                    self.set(1 << Tables::D8, Some(PieceType::Rook));
                    self.black_queenside_castle_rights = false;
                    self.black_kingside_castle_rights = false;
                }
                _ => return,
            }
            self.white_to_move = !self.white_to_move;
            return;
        }
        self.push_state();
        self.clear(play.starting_square, Some(play.moved_type));
        if play.ending_square == self.en_passant_target && play.moved_type == PieceType::Pawn {
            // Special en passant attack logic
            match self.white_to_move {
                true => self.clear_all(play.ending_square >> 8),
                false => self.clear_all(play.ending_square << 8),
            }
        } else {
            // Normal attack clear
            self.clear_all(play.ending_square);
            // If the attacked piece was a rook, remove the relevent castling rights
            if play.ending_square == 1 << Tables::A1 && self.white_queenside_castle_rights {
                self.white_queenside_castle_rights = false;
            } else if play.ending_square == 1 << Tables::H1 && self.white_kingside_castle_rights {
                self.white_kingside_castle_rights = false;
            } else if play.ending_square == 1 << Tables::A8 && self.black_queenside_castle_rights {
                self.black_queenside_castle_rights = false;
            } else if play.ending_square == 1 << Tables::H8 && self.black_kingside_castle_rights {
                self.black_kingside_castle_rights = false;
            }
        }
        // Promotion logic
        if let Some(promotion) = play.promotion {
            match promotion {
                Promotion::Queen => self.set(play.ending_square, Some(PieceType::Queen)),
                Promotion::Rook => self.set(play.ending_square, Some(PieceType::Rook)),
                Promotion::Bishop => self.set(play.ending_square, Some(PieceType::Bishop)),
                Promotion::Knight => self.set(play.ending_square, Some(PieceType::Knight)),
                _ => {}
            }
        } else {
            self.set(play.ending_square, Some(play.moved_type));
        }
        // Do special logic here
        // If the move is not castling, but can effect castling rights, change the rights here
        if play.moved_type == PieceType::Rook {
            if self.white_queenside_castle_rights && play.starting_square == 1 << Tables::A1 {
                self.white_queenside_castle_rights = false;
            }
            if self.white_kingside_castle_rights && play.starting_square == 1 << Tables::H1 {
                self.white_kingside_castle_rights = false;
            }
            if self.black_queenside_castle_rights && play.starting_square == 1 << Tables::A8 {
                self.black_queenside_castle_rights = false;
            }
            if self.black_kingside_castle_rights && play.starting_square == 1 << Tables::H8 {
                self.black_kingside_castle_rights = false;
            }
        }
        if play.moved_type == PieceType::King && play.promotion == None {
            if self.white_to_move {
                self.white_queenside_castle_rights = false;
                self.white_kingside_castle_rights = false;
            } else {
                self.black_queenside_castle_rights = false;
                self.black_kingside_castle_rights = false;
            }
        }
        // Set en passant target
        if play.moved_type == PieceType::Pawn
            && (play.starting_square & Tables::RANK_2 != 0
                && play.ending_square & Tables::RANK_4 != 0
                || play.starting_square & Tables::RANK_7 != 0
                    && play.ending_square & Tables::RANK_5 != 0)
        {
            self.en_passant_target = match self.white_to_move {
                true => play.starting_square << 8,
                false => play.starting_square >> 8,
            }
        } else {
            self.en_passant_target = 0;
        }
        self.white_to_move = !self.white_to_move;
    }

    // Reverts the move from the board. Pops from the move stack
    pub fn unmake(&mut self, play: &MoveRep) {
        self.pop_state();
        // If the move to unmake is castling do this and return
        if play.promotion == Some(Promotion::Castle) {
            // Swap side to play first
            self.white_to_move = !self.white_to_move;
            match play.ending_square {
                e if e == 1 << Tables::G1 => {
                    // White kingside
                    self.set(play.starting_square, Some(PieceType::King));
                    self.set(1 << Tables::H1, Some(PieceType::Rook));
                    self.clear(play.ending_square, Some(PieceType::King));
                    self.clear(1 << Tables::F1, Some(PieceType::Rook));
                    self.white_queenside_castle_rights = true;
                    self.white_kingside_castle_rights = true;
                }
                e if e == 1 << Tables::C1 => {
                    // White queenside
                    self.set(play.starting_square, Some(PieceType::King));
                    self.set(1 << Tables::A1, Some(PieceType::Rook));
                    self.clear(play.ending_square, Some(PieceType::King));
                    self.clear(1 << Tables::D1, Some(PieceType::Rook));
                    self.white_queenside_castle_rights = true;
                    self.white_kingside_castle_rights = true;
                }
                e if e == 1 << Tables::G8 => {
                    // Black kingside
                    self.set(play.starting_square, Some(PieceType::King));
                    self.set(1 << Tables::H8, Some(PieceType::Rook));
                    self.clear(play.ending_square, Some(PieceType::King));
                    self.clear(1 << Tables::F8, Some(PieceType::Rook));
                    self.black_queenside_castle_rights = true;
                    self.black_kingside_castle_rights = true;
                }
                e if e == 1 << Tables::C8 => {
                    // Black queenside
                    self.set(play.starting_square, Some(PieceType::King));
                    self.set(1 << Tables::A8, Some(PieceType::Rook));
                    self.clear(play.ending_square, Some(PieceType::King));
                    self.clear(1 << Tables::D8, Some(PieceType::Rook));
                    self.black_queenside_castle_rights = true;
                    self.black_kingside_castle_rights = true;
                }
                _ => return,
            }
            return;
        }
        if play.ending_square == self.en_passant_target && play.moved_type == PieceType::Pawn {
            // Special en passant attack logic
            // Remember, we have not switch the side to move back yet
            match !self.white_to_move {
                true => self.set(play.ending_square >> 8, play.attacked_type),
                false => self.set(play.ending_square << 8, play.attacked_type),
            }
        } else {
            self.set(play.ending_square, play.attacked_type);
        }
        // Put this after the first set because we want to replace the opponents piece
        self.white_to_move = !self.white_to_move;
        if let Some(promotion) = play.promotion {
            match promotion {
                Promotion::Queen => self.clear(play.ending_square, Some(PieceType::Queen)),
                Promotion::Rook => self.clear(play.ending_square, Some(PieceType::Rook)),
                Promotion::Bishop => self.clear(play.ending_square, Some(PieceType::Bishop)),
                Promotion::Knight => self.clear(play.ending_square, Some(PieceType::Knight)),
                _ => {}
            }
        } else {
            self.clear(play.ending_square, Some(play.moved_type));
        }
        self.set(play.starting_square, Some(play.moved_type));
    }

    // Clear all bitboards at this mask
    #[inline]
    fn clear_all(&mut self, bb: u64) {
        self.white_pawns &= !bb;
        self.white_knights &= !bb;
        self.white_bishops &= !bb;
        self.white_rooks &= !bb;
        self.white_queens &= !bb;
        self.white_king &= !bb;

        self.black_pawns &= !bb;
        self.black_knights &= !bb;
        self.black_bishops &= !bb;
        self.black_rooks &= !bb;
        self.black_queens &= !bb;
        self.black_king &= !bb;
    }

    // Clear bitboards for this value
    #[inline]
    fn clear(&mut self, bb: u64, attacked: Option<PieceType>) {
        if let Some(piece) = attacked {
            if self.white_to_move {
                match piece {
                    PieceType::Pawn => self.white_pawns &= !bb,
                    PieceType::Knight => self.white_knights &= !bb,
                    PieceType::Bishop => self.white_bishops &= !bb,
                    PieceType::Rook => self.white_rooks &= !bb,
                    PieceType::Queen => self.white_queens &= !bb,
                    PieceType::King => self.white_king &= !bb,
                }
            } else {
                match piece {
                    PieceType::Pawn => self.black_pawns &= !bb,
                    PieceType::Knight => self.black_knights &= !bb,
                    PieceType::Bishop => self.black_bishops &= !bb,
                    PieceType::Rook => self.black_rooks &= !bb,
                    PieceType::Queen => self.black_queens &= !bb,
                    PieceType::King => self.black_king &= !bb,
                }
            }
        } else {
            return;
        }
    }

    #[inline]
    fn set(&mut self, bb: u64, present_piece: Option<PieceType>) {
        if let Some(piece) = present_piece {
            if self.white_to_move {
                match piece {
                    PieceType::Pawn => self.white_pawns |= bb,
                    PieceType::Knight => self.white_knights |= bb,
                    PieceType::Bishop => self.white_bishops |= bb,
                    PieceType::Rook => self.white_rooks |= bb,
                    PieceType::Queen => self.white_queens |= bb,
                    PieceType::King => self.white_king |= bb,
                }
            } else {
                match piece {
                    PieceType::Pawn => self.black_pawns |= bb,
                    PieceType::Knight => self.black_knights |= bb,
                    PieceType::Bishop => self.black_bishops |= bb,
                    PieceType::Rook => self.black_rooks |= bb,
                    PieceType::Queen => self.black_queens |= bb,
                    PieceType::King => self.black_king |= bb,
                }
            }
        }
    }

    #[inline]
    pub fn white_occupancy(&self) -> u64 {
        self.white_pawns
            | self.white_knights
            | self.white_bishops
            | self.white_rooks
            | self.white_queens
            | self.white_king
    }

    #[inline]
    pub fn black_occupancy(&self) -> u64 {
        self.black_pawns
            | self.black_knights
            | self.black_bishops
            | self.black_rooks
            | self.black_queens
            | self.black_king
    }

    #[inline]
    pub fn occupancy(&self) -> u64 {
        self.white_pawns
            | self.white_knights
            | self.white_bishops
            | self.white_rooks
            | self.white_queens
            | self.white_king
            | self.black_pawns
            | self.black_knights
            | self.black_bishops
            | self.black_rooks
            | self.black_queens
            | self.black_king
    }

    #[inline]
    // Gets the type of piece present at the mask
    pub fn get_piece_type(&self, mask: u64) -> Option<PieceType> {
        if self.white_pawns & mask != 0 || self.black_pawns & mask != 0 {
            return Some(PieceType::Pawn);
        }
        if self.white_knights & mask != 0 || self.black_knights & mask != 0 {
            return Some(PieceType::Knight);
        }
        if self.white_bishops & mask != 0 || self.black_bishops & mask != 0 {
            return Some(PieceType::Bishop);
        }
        if self.white_rooks & mask != 0 || self.black_rooks & mask != 0 {
            return Some(PieceType::Rook);
        }
        if self.white_queens & mask != 0 || self.black_queens & mask != 0 {
            return Some(PieceType::Queen);
        }
        if self.white_king & mask != 0 || self.black_king & mask != 0 {
            return Some(PieceType::King);
        }

        None
    }

    // Get the attack map of white
    pub fn white_attack_mask(&self, tables: &Tables) -> u64 {
        let mut attack_mask = 0;

        // White pawns
        let mut pawn_bb = self.white_pawns;
        while pawn_bb != 0 {
            let start_square = pop_lsb(&mut pawn_bb);
            attack_mask |= tables.white_pawn_attacks[start_square];
        }

        // White knights
        let mut knight_bb = self.white_knights;
        while knight_bb != 0 {
            let start_square = pop_lsb(&mut knight_bb);
            attack_mask |= tables.knight_attacks[start_square];
        }

        // White bishops
        let mut bishop_bb = self.white_bishops;
        while bishop_bb != 0 {
            let start_square = pop_lsb(&mut bishop_bb);
            let mut bishop_attacks = tables.get_bishop_attack(start_square, self.occupancy());
            while bishop_attacks != 0 {
                let attack_square = 1 << pop_lsb(&mut bishop_attacks) as u64;
                attack_mask |= attack_square;
            }
        }
        // White rooks
        let mut rook_bb = self.white_rooks;
        while rook_bb != 0 {
            let start_square = pop_lsb(&mut rook_bb);
            let mut rook_attacks = tables.get_rook_attack(start_square, self.occupancy());
            while rook_attacks != 0 {
                let attack_square = 1 << pop_lsb(&mut rook_attacks) as u64;
                attack_mask |= attack_square;
            }
        }

        // White queens

        let mut bishop_bb_part = self.white_queens;
        while bishop_bb_part != 0 {
            let start_square = pop_lsb(&mut bishop_bb_part);
            let mut bishop_part_attacks = tables.get_bishop_attack(start_square, self.occupancy());
            while bishop_part_attacks != 0 {
                let attack_square = 1 << pop_lsb(&mut bishop_part_attacks) as u64;
                attack_mask |= attack_square;
            }
        }

        let mut rook_bb_part = self.white_queens;
        while rook_bb_part != 0 {
            let start_square = pop_lsb(&mut rook_bb_part);
            let mut rook_part_attacks = tables.get_rook_attack(start_square, self.occupancy());
            while rook_part_attacks != 0 {
                let attack_square = 1 << pop_lsb(&mut rook_part_attacks) as u64;
                attack_mask |= attack_square;
            }
        }

        // White king
        let mut king_bb = self.white_king;
        while king_bb != 0 {
            let start_square = pop_lsb(&mut king_bb);
            attack_mask |= tables.king_attacks[start_square];
        }

        attack_mask
    }

    // Get the attack map of white with transparency
    pub fn white_attack_mask_with_transparency(&self, tables: &Tables, transparency: u64) -> u64 {
        let mut attack_mask = 0;

        // White pawns
        let mut pawn_bb = self.white_pawns;
        while pawn_bb != 0 {
            let start_square = pop_lsb(&mut pawn_bb);
            attack_mask |= tables.white_pawn_attacks[start_square];
        }

        // White knights
        let mut knight_bb = self.white_knights;
        while knight_bb != 0 {
            let start_square = pop_lsb(&mut knight_bb);
            attack_mask |= tables.knight_attacks[start_square];
        }

        // White bishops
        let mut bishop_bb = self.white_bishops;
        while bishop_bb != 0 {
            let start_square = pop_lsb(&mut bishop_bb);
            let mut bishop_attacks =
                tables.get_bishop_attack(start_square, self.occupancy() & !transparency);
            while bishop_attacks != 0 {
                let attack_square = 1 << pop_lsb(&mut bishop_attacks) as u64;
                attack_mask |= attack_square;
            }
        }
        // White rooks
        let mut rook_bb = self.white_rooks;
        while rook_bb != 0 {
            let start_square = pop_lsb(&mut rook_bb);
            let mut rook_attacks =
                tables.get_rook_attack(start_square, self.occupancy() & !transparency);
            while rook_attacks != 0 {
                let attack_square = 1 << pop_lsb(&mut rook_attacks) as u64;
                attack_mask |= attack_square;
            }
        }

        // White queens

        let mut bishop_bb_part = self.white_queens;
        while bishop_bb_part != 0 {
            let start_square = pop_lsb(&mut bishop_bb_part);
            let mut bishop_part_attacks =
                tables.get_bishop_attack(start_square, self.occupancy() & !transparency);
            while bishop_part_attacks != 0 {
                let attack_square = 1 << pop_lsb(&mut bishop_part_attacks) as u64;
                attack_mask |= attack_square;
            }
        }

        let mut rook_bb_part = self.white_queens;
        while rook_bb_part != 0 {
            let start_square = pop_lsb(&mut rook_bb_part);
            let mut rook_part_attacks =
                tables.get_rook_attack(start_square, self.occupancy() & !transparency);
            while rook_part_attacks != 0 {
                let attack_square = 1 << pop_lsb(&mut rook_part_attacks) as u64;
                attack_mask |= attack_square;
            }
        }

        // White king
        let mut king_bb = self.white_king;
        while king_bb != 0 {
            let start_square = pop_lsb(&mut king_bb);
            attack_mask |= tables.king_attacks[start_square];
        }

        attack_mask
    }

    // Gets the attack mask of only the white leapers
    pub fn white_leaper_attack_mask(&self, tables: &Tables) -> u64 {
        let mut attack_mask = 0;

        // White pawns
        let mut pawn_bb = self.white_pawns;
        while pawn_bb != 0 {
            let start_square = pop_lsb(&mut pawn_bb);
            attack_mask |= tables.white_pawn_attacks[start_square];
        }

        // White knights
        let mut knight_bb = self.white_knights;
        while knight_bb != 0 {
            let start_square = pop_lsb(&mut knight_bb);
            attack_mask |= tables.knight_attacks[start_square];
        }

        attack_mask
    }

    // Get the attack map of black
    pub fn black_attack_mask(&self, table: &Tables) -> u64 {
        let mut attack_mask = 0;

        // black pawns
        let mut pawn_bb = self.black_pawns;
        while pawn_bb != 0 {
            let start_square = pop_lsb(&mut pawn_bb);
            attack_mask |= table.black_pawn_attacks[start_square];
        }

        // black knights
        let mut knight_bb = self.black_knights;
        while knight_bb != 0 {
            let start_square = pop_lsb(&mut knight_bb);
            attack_mask |= table.knight_attacks[start_square];
        }

        // black bishops
        let mut bishop_bb = self.black_bishops;
        while bishop_bb != 0 {
            let start_square = pop_lsb(&mut bishop_bb);
            let mut bishop_attacks = table.get_bishop_attack(start_square, self.occupancy());
            while bishop_attacks != 0 {
                let attack_square = 1 << pop_lsb(&mut bishop_attacks) as u64;
                attack_mask |= attack_square;
            }
        }
        // black rooks
        let mut rook_bb = self.black_rooks;
        while rook_bb != 0 {
            let start_square = pop_lsb(&mut rook_bb);
            let mut rook_attacks = table.get_rook_attack(start_square, self.occupancy());
            while rook_attacks != 0 {
                let attack_square = 1 << pop_lsb(&mut rook_attacks) as u64;
                attack_mask |= attack_square;
            }
        }

        // black queens

        let mut bishop_bb_part = self.black_queens;
        while bishop_bb_part != 0 {
            let start_square = pop_lsb(&mut bishop_bb_part);
            let mut bishop_part_attacks = table.get_bishop_attack(start_square, self.occupancy());
            while bishop_part_attacks != 0 {
                let attack_square = 1 << pop_lsb(&mut bishop_part_attacks) as u64;
                attack_mask |= attack_square;
            }
        }

        let mut rook_bb_part = self.black_queens;
        while rook_bb_part != 0 {
            let start_square = pop_lsb(&mut rook_bb_part);
            let mut rook_part_attacks = table.get_rook_attack(start_square, self.occupancy());
            while rook_part_attacks != 0 {
                let attack_square = 1 << pop_lsb(&mut rook_part_attacks) as u64;
                attack_mask |= attack_square;
            }
        }

        // black king
        let mut king_bb = self.black_king;
        while king_bb != 0 {
            let start_square = pop_lsb(&mut king_bb);
            attack_mask |= table.king_attacks[start_square];
        }

        attack_mask
    }

    // Get the attack map of black with transparency
    pub fn black_attack_mask_with_transparency(&self, table: &Tables, transparency: u64) -> u64 {
        let mut attack_mask = 0;

        // black pawns
        let mut pawn_bb = self.black_pawns;
        while pawn_bb != 0 {
            let start_square = pop_lsb(&mut pawn_bb);
            attack_mask |= table.black_pawn_attacks[start_square];
        }

        // black knights
        let mut knight_bb = self.black_knights;
        while knight_bb != 0 {
            let start_square = pop_lsb(&mut knight_bb);
            attack_mask |= table.knight_attacks[start_square];
        }

        // black bishops
        let mut bishop_bb = self.black_bishops;
        while bishop_bb != 0 {
            let start_square = pop_lsb(&mut bishop_bb);
            let mut bishop_attacks =
                table.get_bishop_attack(start_square, self.occupancy() & !transparency);
            while bishop_attacks != 0 {
                let attack_square = 1 << pop_lsb(&mut bishop_attacks) as u64;
                attack_mask |= attack_square;
            }
        }
        // black rooks
        let mut rook_bb = self.black_rooks;
        while rook_bb != 0 {
            let start_square = pop_lsb(&mut rook_bb);
            let mut rook_attacks =
                table.get_rook_attack(start_square, self.occupancy() & !transparency);
            while rook_attacks != 0 {
                let attack_square = 1 << pop_lsb(&mut rook_attacks) as u64;
                attack_mask |= attack_square;
            }
        }

        // black queens

        let mut bishop_bb_part = self.black_queens;
        while bishop_bb_part != 0 {
            let start_square = pop_lsb(&mut bishop_bb_part);
            let mut bishop_part_attacks =
                table.get_bishop_attack(start_square, self.occupancy() & !transparency);
            while bishop_part_attacks != 0 {
                let attack_square = 1 << pop_lsb(&mut bishop_part_attacks) as u64;
                attack_mask |= attack_square;
            }
        }

        let mut rook_bb_part = self.black_queens;
        while rook_bb_part != 0 {
            let start_square = pop_lsb(&mut rook_bb_part);
            let mut rook_part_attacks =
                table.get_rook_attack(start_square, self.occupancy() & !transparency);
            while rook_part_attacks != 0 {
                let attack_square = 1 << pop_lsb(&mut rook_part_attacks) as u64;
                attack_mask |= attack_square;
            }
        }

        // black king
        let mut king_bb = self.black_king;
        while king_bb != 0 {
            let start_square = pop_lsb(&mut king_bb);
            attack_mask |= table.king_attacks[start_square];
        }

        attack_mask
    }

    // Gets the attack mask of only the black leapers
    pub fn black_leaper_attack_mask(&self, tables: &Tables) -> u64 {
        let mut attack_mask = 0;

        // black pawns
        let mut pawn_bb = self.black_pawns;
        while pawn_bb != 0 {
            let start_square = pop_lsb(&mut pawn_bb);
            attack_mask |= tables.black_pawn_attacks[start_square];
        }

        // black knights
        let mut knight_bb = self.black_knights;
        while knight_bb != 0 {
            let start_square = pop_lsb(&mut knight_bb);
            attack_mask |= tables.knight_attacks[start_square];
        }

        attack_mask
    }

    // Gets the mask of the white pieces that attack the given piece mask
    pub fn white_attacking(&self, tables: &Tables, target: u64) -> u64 {
        // attacking mask
        let mut attacking_mask = 0;
        // turn the piece mask into an index
        let piece_index = target.trailing_zeros() as usize;

        // TODO add en passant attacks

        // Check attacking pawns
        // NOTE this case is diffrent from the rest since pawn moves are not reversible / symetric
        attacking_mask |= tables.black_pawn_attacks[piece_index] & self.white_pawns;
        // Check en passant attacks
        // if self.en_passant_target >> 8 == target {
        //     let en_passant_index = self.en_passant_target.trailing_zeros() as usize;
        //     attacking_mask |= tables.black_pawn_attacks[en_passant_index] & self.white_pawns;
        // }
        // Check attacking knights
        attacking_mask |= tables.knight_attacks[piece_index] & self.white_knights;
        // Check attacking rooks
        attacking_mask |= tables.get_rook_attack(piece_index, self.occupancy()) & self.white_rooks;
        // Check attacking bishops
        attacking_mask |=
            tables.get_bishop_attack(piece_index, self.occupancy()) & self.white_bishops;
        // Check attacking queens
        attacking_mask |= tables.get_rook_attack(piece_index, self.occupancy()) & self.white_queens;
        attacking_mask |=
            tables.get_bishop_attack(piece_index, self.occupancy()) & self.white_queens;
        // Check attacking kings
        attacking_mask |= tables.king_attacks[piece_index] & self.white_king;

        attacking_mask
    }

    // Get the mask of the white pices that 'block' the target. Similar to white_attacking, but with pawn pushes instead of attacks.
    pub fn white_blocking(&self, tables: &Tables, target: u64) -> u64 {
        // blocking mask
        let mut blocking_mask = 0;
        // turn the piece mask into an index
        let piece_index = target.trailing_zeros() as usize;

        // Check blocking pawns
        // NOTE this case is diffrent from the rest since pawn moves are not reversible / symetric
        // Single push
        if target >> 8 & self.white_pawns != 0 {
            blocking_mask |= target >> 8 & self.white_pawns;
        }
        // Double push
        if target >> 16 & self.white_pawns != 0
            && target >> 8 & self.occupancy() == 0
            && target >> 16 & Tables::RANK_2 != 0
        {
            blocking_mask |= target >> 16 & self.white_pawns;
        }
        // Check blocking knights
        blocking_mask |= tables.knight_attacks[piece_index] & self.white_knights;
        // Check blocking rooks
        blocking_mask |= tables.get_rook_attack(piece_index, self.occupancy()) & self.white_rooks;
        // Check blocking bishops
        blocking_mask |=
            tables.get_bishop_attack(piece_index, self.occupancy()) & self.white_bishops;
        // Check blocking queens
        blocking_mask |= tables.get_rook_attack(piece_index, self.occupancy()) & self.white_queens;
        blocking_mask |=
            tables.get_bishop_attack(piece_index, self.occupancy()) & self.white_queens;
        // NOTE No blocking kings because that should never happen

        blocking_mask
    }

    // Gets the mask of the black pieces that attack the given piece mask
    pub fn black_attacking(&self, tables: &Tables, target: u64) -> u64 {
        // attacking mask
        let mut attacking_mask = 0;
        // turn the piece mask into an index
        let piece_index = target.trailing_zeros() as usize;

        // TODO add en passant attacks

        // Check attacking pawns
        // NOTE this case is diffrent from the rest since pawn moves are not reversible / symetric
        attacking_mask |= tables.white_pawn_attacks[piece_index] & self.black_pawns;
        // Check en passant attacks
        // if self.en_passant_target << 8 == target {
        //     let en_passant_index = self.en_passant_target.trailing_zeros() as usize;
        //     attacking_mask |= tables.white_pawn_attacks[en_passant_index] & self.black_pawns;
        // }
        // Check attacking knights
        attacking_mask |= tables.knight_attacks[piece_index] & self.black_knights;
        // Check attacking rooks
        attacking_mask |= tables.get_rook_attack(piece_index, self.occupancy()) & self.black_rooks;
        // Check attacking bishops
        attacking_mask |=
            tables.get_bishop_attack(piece_index, self.occupancy()) & self.black_bishops;
        // Check attacking queens
        attacking_mask |= tables.get_rook_attack(piece_index, self.occupancy()) & self.black_queens;
        attacking_mask |=
            tables.get_bishop_attack(piece_index, self.occupancy()) & self.black_queens;
        // Check attacking kings
        attacking_mask |= tables.king_attacks[piece_index] & self.black_king;

        attacking_mask
    }

    // Get the mask of the black pices that 'block' the target. Similar to black_attacking, but with pawn pushes instead of attacks.
    pub fn black_blocking(&self, tables: &Tables, target: u64) -> u64 {
        // blocking mask
        let mut blocking_mask = 0;
        // turn the piece mask into an index
        let piece_index = target.trailing_zeros() as usize;

        // Check blocking pawns
        // NOTE this case is diffrent from the rest since pawn moves are not reversible / symetric
        // Single push
        if target << 8 & self.black_pawns != 0 {
            blocking_mask |= target << 8 & self.black_pawns;
        }
        // Double push
        if target << 16 & self.black_pawns != 0
            && target << 8 & self.occupancy() == 0
            && target << 16 & Tables::RANK_7 != 0
        {
            blocking_mask |= target << 16 & self.black_pawns;
        }
        // Check blocking knights
        blocking_mask |= tables.knight_attacks[piece_index] & self.black_knights;
        // Check blocking rooks
        blocking_mask |= tables.get_rook_attack(piece_index, self.occupancy()) & self.black_rooks;
        // Check blocking bishops
        blocking_mask |=
            tables.get_bishop_attack(piece_index, self.occupancy()) & self.black_bishops;
        // Check blocking queens
        blocking_mask |= tables.get_rook_attack(piece_index, self.occupancy()) & self.black_queens;
        blocking_mask |=
            tables.get_bishop_attack(piece_index, self.occupancy()) & self.black_queens;
        // NOTE No blocking kings because that should never happen

        blocking_mask
    }

    /// Gets the mask of the pieces pinned to the target
    pub fn pin_mask(&self, tables: &Tables, target: u64, white_to_move: bool) -> u64 {
        let target_index = target.trailing_zeros() as usize;
        let mask = match white_to_move {
            true => {
                let mut mask = 0;
                // Project a rook ray without any blockers
                let rook_ray =
                    tables.get_rook_attack(target_index, self.black_rooks | self.black_queens);
                // Get the pieces that could attack if there were no blockers
                let mut connected_pieces = rook_ray & (self.black_rooks | self.black_queens);
                while connected_pieces != 0 {
                    let attacker_index = pop_lsb(&mut connected_pieces);
                    // Look back to the target without any blockers
                    let attacker_ray = tables.get_rook_attack(attacker_index, target);
                    // Get the mask which must protect the target
                    let blocker_mask = rook_ray & attacker_ray;
                    // If there is only one piece in the way, add it to the mask if its the current sides color
                    if (blocker_mask & self.occupancy()).count_ones() == 1 {
                        mask |= blocker_mask & self.white_occupancy();
                    }
                }

                // Now do the same thing with bishop rays
                // Project a bishop ray without any blockers
                let bishop_ray =
                    tables.get_bishop_attack(target_index, self.black_bishops | self.black_queens);
                // Get the pieces that could attack if there were no blockers
                let mut connected_pieces = bishop_ray & (self.black_bishops | self.black_queens);
                while connected_pieces != 0 {
                    let attacker_index = pop_lsb(&mut connected_pieces);
                    // Look back to the target without any blockers
                    let attacker_ray = tables.get_bishop_attack(attacker_index, target);
                    // Get the mask which must protect the target
                    let blocker_mask = bishop_ray & attacker_ray;
                    // If there is only one piece in the way, add it to the mask if its the current sides color
                    if (blocker_mask & self.occupancy()).count_ones() == 1 {
                        mask |= blocker_mask & self.white_occupancy();
                    }
                }
                // Return the mask
                mask
            }
            false => {
                let mut mask = 0;
                // Project a rook ray without any blockers
                let rook_ray =
                    tables.get_rook_attack(target_index, self.white_rooks | self.white_queens);
                // Get the pieces that could attack if there were no blockers
                let mut connected_pieces = rook_ray & (self.white_rooks | self.white_queens);
                while connected_pieces != 0 {
                    let attacker_index = pop_lsb(&mut connected_pieces);
                    // Look back to the target without any blockers
                    let attacker_ray = tables.get_rook_attack(attacker_index, target);
                    // Get the mask which must protect the target
                    let blocker_mask = rook_ray & attacker_ray;
                    // If there is only one piece in the way, add it to the mask if its the current sides color
                    if (blocker_mask & self.occupancy()).count_ones() == 1 {
                        mask |= blocker_mask & self.black_occupancy();
                    }
                }

                // Now do the same thing with bishop rays
                // Project a bishop ray without any blockers
                let bishop_ray =
                    tables.get_bishop_attack(target_index, self.white_bishops | self.white_queens);
                // Get the pieces that could attack if there were no blockers
                let mut connected_pieces = bishop_ray & (self.white_bishops | self.white_queens);
                while connected_pieces != 0 {
                    let attacker_index = pop_lsb(&mut connected_pieces);
                    // Look back to the target without any blockers
                    let attacker_ray = tables.get_bishop_attack(attacker_index, target);
                    // Get the mask which must protect the target
                    let blocker_mask = bishop_ray & attacker_ray;
                    // If there is only one piece in the way, add it to the mask if its the current sides color
                    if (blocker_mask & self.occupancy()).count_ones() == 1 {
                        mask |= blocker_mask & self.black_occupancy();
                    }
                }
                // Return the mask
                mask
            }
        };
        mask
    }

    // Tests if the target is safe from a ray attack after the move rep
    pub fn pin_safe(&self, tables: &Tables, target: u64, mv: &MoveRep) -> bool {
        // TODO Add special handling for en passant moves, because they can reveal an attack!
        // The occupancy after the move would be made
        let after_occupancy;
        // Special en passant logic
        if mv.ending_square == self.en_passant_target {
            let en_passant_attacked = match self.white_to_move {
                true => mv.ending_square >> 8,
                false => mv.ending_square << 8,
            };
            after_occupancy =
                self.occupancy() & !mv.starting_square & !en_passant_attacked | mv.ending_square;
        } else {
            // Normal case
            after_occupancy = self.occupancy() & !mv.starting_square | mv.ending_square;
        }
        let target_index = target.trailing_zeros() as usize;
        // A move which attacks the attacker is safe, unless the attackers space is also under attack

        // Get the relevent attackers, and remove them if they are attacked by the move
        let rook_like_mask = match self.white_to_move {
            true => (self.black_rooks | self.black_queens) & !mv.ending_square,
            false => (self.white_rooks | self.white_queens) & !mv.ending_square,
        };
        let bishop_like_mask = match self.white_to_move {
            true => (self.black_bishops | self.black_queens) & !mv.ending_square,
            false => (self.white_bishops | self.white_queens) & !mv.ending_square,
        };

        // Project rays from the target and check if the target could be attacked
        let rook_ray = tables.get_rook_attack(target_index, after_occupancy);
        if rook_ray & rook_like_mask != 0 {
            return false;
        }
        let bishop_ray = tables.get_bishop_attack(target_index, after_occupancy);
        if bishop_ray & bishop_like_mask != 0 {
            return false;
        }

        true
    }

    // Get if the white king is in check
    pub fn white_in_check(&self, table: &Tables) -> bool {
        let black_attack_mask = self.black_attack_mask(table);
        black_attack_mask & self.white_king != 0
    }

    // Get if the black king is in check
    pub fn black_in_check(&self, table: &Tables) -> bool {
        let white_attack_mask = self.white_attack_mask(table);
        white_attack_mask & self.black_king != 0
    }

    // Checks that the move will not result in check
    pub fn move_safe_for_king(&mut self, table: &Tables, play: &MoveRep) -> bool {
        self.make(&play);
        let is_safe = !match self.white_to_move {
            true => self.black_in_check(&table),
            false => self.white_in_check(&table),
        };
        self.unmake(&play);
        is_safe
    }
}

impl MoveRep {
    pub fn to_string(&self) -> Result<String, String> {
        let start = self.starting_square;
        let end = self.ending_square;
        let mut mov = String::new();
        mov.push_str(MoveRep::mask_to_string(start).unwrap().as_ref());
        mov.push_str(MoveRep::mask_to_string(end).unwrap().as_ref());
        if self.promotion.is_some() && self.promotion != Some(Promotion::Castle) {
            match self.promotion {
                Some(Promotion::Queen) => mov.push_str("q"),
                Some(Promotion::Bishop) => mov.push_str("b"),
                Some(Promotion::Rook) => mov.push_str("r"),
                Some(Promotion::Knight) => mov.push_str("n"),
                _ => {}
            }
        }
        Ok(mov)
    }

    /// Returns if the move is reversible
    pub fn is_reversible(&self) -> bool {
        // If there is a piece captured, it is not reversible
        if self.attacked_type.is_some() {
            return false;
        }

        // If there is a promotion, it is not reversible
        if self.promotion.is_some() {
            return false;
        }

        // If the piece is a pawn, it is not reversible
        if self.moved_type == PieceType::Pawn {
            return false;
        }

        // If the move is a castle, is is not reversible
        if self.moved_type == PieceType::King
            && (self.starting_square == 1 << Tables::E1 && self.ending_square == 1 << Tables::A1)
            || (self.starting_square == 1 << Tables::E1 && self.ending_square == 1 << Tables::H1)
            || (self.starting_square == 1 << Tables::E8 && self.ending_square == 1 << Tables::A8)
            || (self.starting_square == 1 << Tables::E8 && self.ending_square == 1 << Tables::H8)
        {
            return false;
        }

        // If we fell through the above conditions, the move is reversible
        true
    }

    pub fn mask_to_string(mask: u64) -> Result<String, String> {
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

pub fn print_bitboard(bb: u64) {
    fn get_bit(bb: u64, index: u64) -> char {
        match bb & 1 << index {
            0 => '0',
            _ => '1',
        }
    }
    println!(
        "8   {} {} {} {} {} {} {} {}",
        get_bit(bb, 63),
        get_bit(bb, 62),
        get_bit(bb, 61),
        get_bit(bb, 60),
        get_bit(bb, 59),
        get_bit(bb, 58),
        get_bit(bb, 57),
        get_bit(bb, 56)
    );

    println!(
        "7   {} {} {} {} {} {} {} {}",
        get_bit(bb, 55),
        get_bit(bb, 54),
        get_bit(bb, 53),
        get_bit(bb, 52),
        get_bit(bb, 51),
        get_bit(bb, 50),
        get_bit(bb, 49),
        get_bit(bb, 48)
    );

    println!(
        "6   {} {} {} {} {} {} {} {}",
        get_bit(bb, 47),
        get_bit(bb, 46),
        get_bit(bb, 45),
        get_bit(bb, 44),
        get_bit(bb, 43),
        get_bit(bb, 42),
        get_bit(bb, 41),
        get_bit(bb, 40)
    );

    println!(
        "5   {} {} {} {} {} {} {} {}",
        get_bit(bb, 39),
        get_bit(bb, 38),
        get_bit(bb, 37),
        get_bit(bb, 36),
        get_bit(bb, 35),
        get_bit(bb, 34),
        get_bit(bb, 33),
        get_bit(bb, 32)
    );

    println!(
        "4   {} {} {} {} {} {} {} {}",
        get_bit(bb, 31),
        get_bit(bb, 30),
        get_bit(bb, 29),
        get_bit(bb, 28),
        get_bit(bb, 27),
        get_bit(bb, 26),
        get_bit(bb, 25),
        get_bit(bb, 24)
    );

    println!(
        "3   {} {} {} {} {} {} {} {}",
        get_bit(bb, 23),
        get_bit(bb, 22),
        get_bit(bb, 21),
        get_bit(bb, 20),
        get_bit(bb, 19),
        get_bit(bb, 18),
        get_bit(bb, 17),
        get_bit(bb, 16)
    );

    println!(
        "2   {} {} {} {} {} {} {} {}",
        get_bit(bb, 15),
        get_bit(bb, 14),
        get_bit(bb, 13),
        get_bit(bb, 12),
        get_bit(bb, 11),
        get_bit(bb, 10),
        get_bit(bb, 9),
        get_bit(bb, 8)
    );

    println!(
        "1   {} {} {} {} {} {} {} {}",
        get_bit(bb, 7),
        get_bit(bb, 6),
        get_bit(bb, 5),
        get_bit(bb, 4),
        get_bit(bb, 3),
        get_bit(bb, 2),
        get_bit(bb, 1),
        get_bit(bb, 0)
    );

    println!("\n    a b c d e f g h");
    io::stdout().flush();
}

#[cfg(test)]
mod tests {
    use core::panic::PanicInfo;
    use std::fmt::Debug;

    use crate::{generate, tables::Tables};

    use super::*;

    #[test]
    fn test_white_pawn_push() {
        let mut pawn_test = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        let move_test = MoveRep {
            starting_square: 1 << Tables::A2,
            ending_square: 1 << Tables::A4,
            promotion: None,
            moved_type: PieceType::Pawn,
            attacked_type: None,
        };

        pawn_test.make(&move_test);

        assert_eq!(pawn_test.white_to_move, false);
        assert_eq!(pawn_test.white_pawns & 1 << Tables::A4 != 0, true);
        assert_eq!(pawn_test.white_pawns & 1 << Tables::A2 != 0, false);
    }

    #[test]
    fn test_black_pawn_push() {
        let mut black_pawn_test = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );

        let move_test = MoveRep {
            starting_square: 1 << Tables::D7,
            ending_square: 1 << Tables::D5,
            promotion: None,
            moved_type: PieceType::Pawn,
            attacked_type: None,
        };

        black_pawn_test.make(&move_test);

        assert_eq!(black_pawn_test.white_to_move, true);
        assert_eq!(black_pawn_test.black_pawns & 1 << Tables::D7 != 0, false);
        assert_eq!(black_pawn_test.black_pawns & 1 << Tables::D5 != 0, true);
    }

    #[test]
    fn test_black_pawn_attack() {
        let mut black_pawn_attack_test = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/1P6/8/8/8/P1PPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );

        let move_test = MoveRep {
            starting_square: 1 << Tables::A7,
            ending_square: 1 << Tables::B6,
            promotion: None,
            moved_type: PieceType::Pawn,
            attacked_type: Some(PieceType::Pawn),
        };

        black_pawn_attack_test.make(&move_test);

        assert_eq!(black_pawn_attack_test.white_to_move, true);
        assert_eq!(
            black_pawn_attack_test.black_pawns & 1 << Tables::A7 != 0,
            false
        );
        assert_eq!(
            black_pawn_attack_test.black_pawns & 1 << Tables::B6 != 0,
            true
        );
        assert_eq!(
            black_pawn_attack_test.white_pawns & 1 << Tables::B6 != 0,
            false
        );
    }

    #[test]
    fn test_white_pawn_attack() {
        let mut pawn_attack_test = BoardState::state_from_string_fen(
            "rnbqkbnr/1ppppppp/8/8/8/2p5/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        let move_test = MoveRep {
            starting_square: 1 << Tables::B2,
            ending_square: 1 << Tables::C3,
            promotion: None,
            moved_type: PieceType::Pawn,
            attacked_type: Some(PieceType::Pawn),
        };

        pawn_attack_test.make(&move_test);

        assert_eq!(pawn_attack_test.white_to_move, false);
        assert_eq!(pawn_attack_test.white_pawns & 1 << Tables::B2 != 0, false);
        assert_eq!(pawn_attack_test.white_pawns & 1 << Tables::C3 != 0, true);
        assert_eq!(pawn_attack_test.black_pawns & 1 << Tables::C3 != 0, false);
    }

    #[test]
    fn test_white_knight() {
        let mut knight_test = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/N7/PPPPPPPP/R1BQKBNR w KQkq - 0 1".to_string(),
        );

        let move_test = MoveRep {
            starting_square: 1 << Tables::B2,
            ending_square: 1 << Tables::A3,
            promotion: None,
            moved_type: PieceType::Knight,
            attacked_type: None,
        };

        knight_test.make(&move_test);

        assert_eq!(knight_test.white_to_move, false);
        assert_eq!(knight_test.white_knights & 1 << Tables::A3 != 0, true);
        assert_eq!(knight_test.white_knights & 1 << Tables::B2 != 0, false);
    }

    #[test]
    fn test_black_knight() {
        let mut black_knight_test = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );

        let move_test = MoveRep {
            starting_square: 1 << Tables::B8,
            ending_square: 1 << Tables::A6,
            promotion: None,
            moved_type: PieceType::Knight,
            attacked_type: None,
        };

        black_knight_test.make(&move_test);

        assert_eq!(black_knight_test.white_to_move, true);
        assert_eq!(
            black_knight_test.black_knights & 1 << Tables::B8 != 0,
            false
        );
        assert_eq!(black_knight_test.black_knights & 1 << Tables::A6 != 0, true);
    }

    #[test]
    fn test_white_knight_attack() {
        let mut white_knight_attack = BoardState::state_from_string_fen(
            "rnbqkbnr/1ppppppp/8/8/8/2p5/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        let move_test = MoveRep {
            starting_square: 1 << Tables::B1,
            ending_square: 1 << Tables::C3,
            promotion: None,
            moved_type: PieceType::Knight,
            attacked_type: Some(PieceType::Pawn),
        };

        white_knight_attack.make(&move_test);

        assert_eq!(white_knight_attack.white_to_move, false);
        assert_eq!(
            white_knight_attack.white_knights & 1 << Tables::B1 != 0,
            false
        );
        assert_eq!(
            white_knight_attack.white_knights & 1 << Tables::C3 != 0,
            true
        );
        assert_eq!(
            white_knight_attack.black_pawns & 1 << Tables::C3 != 0,
            false
        );
    }

    #[test]
    fn test_black_knight_attack() {
        let mut black_knight_attack_test = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/2P5/8/8/8/P1PPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );

        let move_test = MoveRep {
            starting_square: 1 << Tables::B8,
            ending_square: 1 << Tables::C6,
            promotion: None,
            moved_type: PieceType::Knight,
            attacked_type: Some(PieceType::Pawn),
        };

        black_knight_attack_test.make(&move_test);

        assert_eq!(black_knight_attack_test.white_to_move, true);
        assert_eq!(
            black_knight_attack_test.black_knights & 1 << Tables::B8 != 0,
            false
        );
        assert_eq!(
            black_knight_attack_test.black_knights & 1 << Tables::C6 != 0,
            true
        );
        assert_eq!(
            black_knight_attack_test.white_pawns & 1 << Tables::C6 != 0,
            false
        );
    }

    #[test]
    fn white_rook_move() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/1PPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        let move_test = MoveRep {
            starting_square: 1 << Tables::A1,
            ending_square: 1 << Tables::A5,
            promotion: None,
            moved_type: PieceType::Rook,
            attacked_type: None,
        };

        board.make(&move_test);

        assert_eq!(board.white_to_move, false);
        assert_eq!(board.white_rooks & 1 << Tables::A1 != 0, false);
        assert_eq!(board.white_rooks & 1 << Tables::A5 != 0, true);
    }

    #[test]
    fn black_rook_move() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/1ppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );

        let move_test = MoveRep {
            starting_square: 1 << Tables::A8,
            ending_square: 1 << Tables::A3,
            promotion: None,
            moved_type: PieceType::Rook,
            attacked_type: None,
        };

        board.make(&move_test);

        assert_eq!(board.white_to_move, true);
        assert_eq!(board.black_rooks & 1 << Tables::A8 != 0, false);
        assert_eq!(board.black_rooks & 1 << Tables::A3 != 0, true);
    }

    #[test]
    fn white_rook_attack() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/p7/8/1PPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        let move_test = MoveRep {
            starting_square: 1 << Tables::A1,
            ending_square: 1 << Tables::A4,
            promotion: None,
            moved_type: PieceType::Rook,
            attacked_type: Some(PieceType::Pawn),
        };

        board.make(&move_test);

        assert_eq!(board.white_to_move, false);
        assert_eq!(board.white_rooks & 1 << Tables::A1 != 0, false);
        assert_eq!(board.white_rooks & 1 << Tables::A4 != 0, true);
        assert_eq!(board.black_pawns & 1 << Tables::A4 != 0, false);
    }

    #[test]
    fn black_rook_attack() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/1ppppppp/8/8/P7/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );

        let move_test = MoveRep {
            starting_square: 1 << Tables::A8,
            ending_square: 1 << Tables::A4,
            promotion: None,
            moved_type: PieceType::Rook,
            attacked_type: Some(PieceType::Pawn),
        };

        board.make(&move_test);

        assert_eq!(board.white_to_move, true);
        assert_eq!(board.black_rooks & 1 << Tables::A8 != 0, false);
        assert_eq!(board.black_rooks & 1 << Tables::A4 != 0, true);
        assert_eq!(board.white_pawns & 1 << Tables::A4 != 0, false);
    }

    #[test]
    fn unmake_white_pawn_push() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        let move_test = MoveRep {
            starting_square: 1 << Tables::D2,
            ending_square: 1 << Tables::D4,
            promotion: None,
            moved_type: PieceType::Pawn,
            attacked_type: None,
        };

        board.make(&move_test);
        board.unmake(&move_test);

        assert_eq!(board.white_to_move, true);
        assert_eq!(board.white_pawns & 1 << Tables::D2 != 0, true);
        assert_eq!(board.white_pawns & 1 << Tables::D4 != 0, false);
    }

    #[test]
    fn unmake_white_pawn_attack() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pp1ppppp/8/8/8/2p5/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        let move_test = MoveRep {
            starting_square: 1 << Tables::D2,
            ending_square: 1 << Tables::C3,
            promotion: None,
            moved_type: PieceType::Pawn,
            attacked_type: Some(PieceType::Pawn),
        };

        board.make(&move_test);
        board.unmake(&move_test);

        assert_eq!(board.white_to_move, true);
        assert_eq!(board.white_pawns & 1 << Tables::D2 != 0, true);
        assert_eq!(board.white_pawns & 1 << Tables::C3 != 0, false);
        print_bitboard(board.black_pawns);
        assert_eq!(board.black_pawns & 1 << Tables::C3 != 0, true);
    }

    #[test]
    fn unmake_black_pawn_push() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );

        let move_test = MoveRep {
            starting_square: 1 << Tables::H7,
            ending_square: 1 << Tables::H5,
            promotion: None,
            moved_type: PieceType::Pawn,
            attacked_type: None,
        };

        board.make(&move_test);
        board.unmake(&move_test);

        assert_eq!(board.white_to_move, false);
        assert_eq!(board.black_pawns & 1 << Tables::H7 != 0, true);
        assert_eq!(board.black_pawns & 1 << Tables::H5 != 0, false);
    }

    #[test]
    fn unmake_black_pawn_attack() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/P7/8/8/8/PP1PPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );

        let move_test = MoveRep {
            starting_square: 1 << Tables::B7,
            ending_square: 1 << Tables::A6,
            promotion: None,
            moved_type: PieceType::Pawn,
            attacked_type: Some(PieceType::Pawn),
        };

        board.make(&move_test);
        board.unmake(&move_test);

        assert_eq!(board.white_to_move, false);
        assert_eq!(board.black_pawns & 1 << Tables::B7 != 0, true);
        assert_eq!(board.black_pawns & 1 << Tables::A6 != 0, false);
        assert_eq!(board.white_pawns & 1 << Tables::A6 != 0, true);
    }

    #[test]
    fn unmake_white_knight_move() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        let test_move = MoveRep {
            starting_square: 1 << Tables::G1,
            ending_square: 1 << Tables::H3,
            promotion: None,
            moved_type: PieceType::Knight,
            attacked_type: None,
        };

        board.make(&test_move);
        board.unmake(&test_move);

        assert_eq!(board.white_to_move, true);
        assert_eq!(board.white_knights & 1 << Tables::G1 != 0, true);
        assert_eq!(board.white_knights & 1 << Tables::H3 != 0, false);
    }

    #[test]
    fn unmake_white_knight_attack() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pp1ppppp/8/8/8/5p2/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        let test_move = MoveRep {
            starting_square: 1 << Tables::G1,
            ending_square: 1 << Tables::H3,
            promotion: None,
            moved_type: PieceType::Knight,
            attacked_type: Some(PieceType::Pawn),
        };

        board.make(&test_move);
        board.unmake(&test_move);

        assert_eq!(board.white_to_move, true);
        assert_eq!(board.white_knights & 1 << Tables::G1 != 0, true);
        assert_eq!(board.white_knights & 1 << Tables::H3 != 0, false);
        assert_eq!(board.black_pawns & 1 << Tables::H3 != 0, true);
    }

    #[test]
    fn unmake_black_knight_move() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );

        let test_move = MoveRep {
            starting_square: 1 << Tables::G8,
            ending_square: 1 << Tables::F6,
            promotion: None,
            moved_type: PieceType::Knight,
            attacked_type: None,
        };

        board.make(&test_move);
        board.unmake(&test_move);

        assert_eq!(board.white_to_move, false);
        assert_eq!(board.black_knights & 1 << Tables::G8 != 0, true);
        assert_eq!(board.black_knights & 1 << Tables::F6 != 0, false);
    }

    #[test]
    fn unmake_black_knight_attack() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/7P/8/8/8/PPPPP1PP/RNBQKBNR b KQkq - 0 1".to_string(),
        );

        let test_move = MoveRep {
            starting_square: 1 << Tables::G8,
            ending_square: 1 << Tables::H6,
            promotion: None,
            moved_type: PieceType::Knight,
            attacked_type: Some(PieceType::Pawn),
        };

        board.make(&test_move);
        board.unmake(&test_move);

        assert_eq!(board.white_to_move, false);
        assert_eq!(board.black_knights & 1 << Tables::G8 != 0, true);
        assert_eq!(board.black_knights & 1 << Tables::H6 != 0, false);
        assert_eq!(board.white_pawns & 1 << Tables::H6 != 0, true);
    }

    #[test]
    fn unmake_white_rook_move() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/1PPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        let test_move = MoveRep {
            starting_square: 1 << Tables::A1,
            ending_square: 1 << Tables::A5,
            promotion: None,
            moved_type: PieceType::Rook,
            attacked_type: None,
        };

        board.make(&test_move);
        board.unmake(&test_move);

        assert_eq!(board.white_to_move, true);
        assert_eq!(board.white_rooks & 1 << Tables::A1 != 0, true);
        assert_eq!(board.white_rooks & 1 << Tables::A5 != 0, false);
    }

    #[test]
    fn unmake_white_rook_attack() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/p7/8/8/1PPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        let test_move = MoveRep {
            starting_square: 1 << Tables::A1,
            ending_square: 1 << Tables::A5,
            promotion: None,
            moved_type: PieceType::Rook,
            attacked_type: Some(PieceType::Pawn),
        };

        board.make(&test_move);
        board.unmake(&test_move);

        assert_eq!(board.white_to_move, true);
        assert_eq!(board.white_rooks & 1 << Tables::A1 != 0, true);
        assert_eq!(board.white_rooks & 1 << Tables::A5 != 0, false);
        assert_eq!(board.black_pawns & 1 << Tables::A5 != 0, true);
    }

    #[test]
    fn unmake_black_rook_move() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/1ppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );

        let test_move = MoveRep {
            starting_square: 1 << Tables::A8,
            ending_square: 1 << Tables::A3,
            promotion: None,
            moved_type: PieceType::Rook,
            attacked_type: None,
        };

        board.make(&test_move);
        board.unmake(&test_move);

        assert_eq!(board.white_to_move, false);
        assert_eq!(board.black_rooks & 1 << Tables::A8 != 0, true);
        assert_eq!(board.black_rooks & 1 << Tables::A3 != 0, false);
    }

    #[test]
    fn unmake_black_rook_attack() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/1ppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );

        let test_move = MoveRep {
            starting_square: 1 << Tables::A8,
            ending_square: 1 << Tables::A2,
            promotion: None,
            moved_type: PieceType::Rook,
            attacked_type: Some(PieceType::Pawn),
        };

        board.make(&test_move);
        board.unmake(&test_move);

        assert_eq!(board.white_to_move, false);
        assert_eq!(board.black_rooks & 1 << Tables::A8 != 0, true);
        assert_eq!(board.black_rooks & 1 << Tables::A2 != 0, false);
        assert_eq!(board.white_pawns & 1 << Tables::A2 != 0, true);
    }

    #[test]
    fn white_attack_mask_pawn() {
        let board = BoardState::state_from_string_fen("8/8/8/8/8/8/7P/8 w - - 0 1".to_string());
        let tables = Tables::new();
        let expected = 0x20000;
        let result = board.white_attack_mask(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn white_attack_mask_knight() {
        let board = BoardState::state_from_string_fen("8/8/8/3N4/8/8/8/8 w - - 0 1".to_string());
        let tables = Tables::new();
        let expected = 0x28440044280000;
        let result = board.white_attack_mask(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn white_attack_mask_rook() {
        let board = BoardState::state_from_string_fen("8/8/8/8/8/8/8/3R4 w - - 0 1".to_string());
        let tables = Tables::new();
        let expected = 0x10101010101010ef;
        let result = board.white_attack_mask(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn white_attack_mask_rook_with_blocker() {
        let board = BoardState::state_from_string_fen("8/8/8/8/3P4/8/8/3R4 w - - 0 1".to_string());
        let tables = Tables::new();
        let expected = 0x28101010ef;
        let result = board.white_attack_mask(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn white_attack_mask_bishop() {
        let board = BoardState::state_from_string_fen("8/8/8/8/8/8/8/5B2 w - - 0 1".to_string());
        let tables = Tables::new();
        let expected = 0x804020110a00;
        let result = board.white_attack_mask(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn white_attack_mask_bishop_with_blocker() {
        let board = BoardState::state_from_string_fen("8/8/8/8/2P5/8/8/5B2 w - - 0 1".to_string());
        let tables = Tables::new();
        let expected = 0x5020110a00;
        let result = board.white_attack_mask(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn white_attack_mask_queen() {
        let board = BoardState::state_from_string_fen("8/8/8/8/3Q4/8/8/8 w - - 0 1".to_string());
        let tables = Tables::new();
        let expected = 0x11925438ef385492;
        let result = board.white_attack_mask(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn white_attack_mask_starting_pos() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let expected = 0xffff7e;
        let result = board.white_attack_mask(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn black_attack_mask_pawn() {
        let board = BoardState::state_from_string_fen("8/8/3p4/8/8/8/8/8 b - - 0 1".to_string());
        let tables = Tables::new();
        let expected = 0x2800000000;
        let result = board.black_attack_mask(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn black_attack_mask_knight() {
        let board = BoardState::state_from_string_fen("8/8/3n4/8/8/8/8/8 b - - 0 1".to_string());
        let tables = Tables::new();
        let expected = 0x2844004428000000;
        let result = board.black_attack_mask(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn black_attack_mask_rook() {
        let board = BoardState::state_from_string_fen("8/8/8/3r4/8/8/8/8 b - - 0 1".to_string());
        let tables = Tables::new();
        let expected = 0x101010ef10101010;
        let result = board.black_attack_mask(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn black_attack_mask_rook_with_blocker() {
        let board = BoardState::state_from_string_fen("8/8/8/3r1p2/8/8/8/8 b - - 0 1".to_string());
        let tables = Tables::new();
        let expected = 0x101010ec1a101010;
        let result = board.black_attack_mask(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn black_attack_mask_bishop() {
        let board = BoardState::state_from_string_fen("8/8/8/3b4/8/8/8/8 b - - 0 1".to_string());
        let tables = Tables::new();
        let expected = 0x8244280028448201;
        let result = board.black_attack_mask(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn black_attack_mask_bishop_with_blocker() {
        let board = BoardState::state_from_string_fen("8/8/8/8/2p5/8/8/5b2 b - - 0 1".to_string());
        let tables = Tables::new();
        let expected = 0x20510a00;
        let result = board.black_attack_mask(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn black_attack_mask_queen() {
        let board = BoardState::state_from_string_fen("8/8/8/8/3q4/8/8/8 w - - 0 1".to_string());
        let tables = Tables::new();
        let expected = 0x11925438ef385492;
        let result = board.black_attack_mask(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn white_in_check_1() {
        let board = BoardState::state_from_string_fen(
            "rnb1kbnr/pp1ppppp/2p5/q7/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let expected = true;
        let result = board.white_in_check(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn white_in_check_2() {
        let board = BoardState::state_from_string_fen(
            "rnbqkb1r/pppppppp/8/8/8/5n2/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let expected = true;
        let result = board.white_in_check(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn white_in_check_3() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let expected = false;
        let result = board.white_in_check(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn white_in_check_4() {
        let board = BoardState::state_from_string_fen(
            "rnb1kbnr/pppppppp/8/8/4q3/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let expected = false;
        let result = board.white_in_check(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn black_in_check_1() {
        let board = BoardState::state_from_string_fen(
            "rnbq1bnr/ppppkppp/8/3Np3/8/8/PPPPPPPP/R1BQKBNR b KQ - 0 1".to_string(),
        );
        let tables = Tables::new();
        let expected = true;
        let result = board.black_in_check(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn black_in_check_2() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/3N4/8/8/8/PPPPPPPP/R1BQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let expected = true;
        let result = board.black_in_check(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn black_in_check_3() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let expected = false;
        let result = board.black_in_check(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn black_in_check_4() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/3Q4/8/8/PPPPPPPP/RNB1KBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let expected = false;
        let result = board.black_in_check(&tables);
        assert_eq!(expected, result);
    }

    #[test]
    fn white_attacking_1() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppp1ppp/8/4Q3/8/8/PPPPPPPP/RNB1KBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let expected = 1 << Tables::E5;
        let result = board.white_attacking(&tables, 1 << Tables::E8);
        assert_eq!(expected, result);
    }

    #[test]
    fn white_attacking_2() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        let tables = Tables::new();
        let expected = 0;
        let result = board.white_attacking(&tables, 1 << Tables::E8);
        assert_eq!(expected, result);
    }

    #[test]
    fn white_attacking_3() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );

        let tables = Tables::new();
        let expected = 0x78;
        let result = board.white_attacking(&tables, 1 << Tables::D2);
        assert_eq!(expected, result);
    }

    #[test]
    fn white_attacking_4() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/3N4/8/8/8/PPPPPPPP/RNBQKB1R w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let expected = 1 << Tables::D6;
        let result = board.white_attacking(&tables, 1 << Tables::C8);
        assert_eq!(expected, result);
    }

    #[test]
    fn white_attacking_5() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/3P4/8/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let expected = 1 << Tables::D6;
        let result = board.white_attacking(&tables, 1 << Tables::E7);
        assert_eq!(expected, result);
    }

    #[test]
    fn black_attacking_1() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/ppp1pppp/8/8/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let expected = 1 << Tables::D8;
        let result = board.black_attacking(&tables, 1 << Tables::D1);
        assert_eq!(expected, result);
    }

    #[test]
    fn black_attacking_2() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppp1ppp/8/8/8/4p3/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let expected = 1 << Tables::E3;
        let result = board.black_attacking(&tables, 1 << Tables::F2);
        assert_eq!(expected, result);
    }

    #[test]
    fn black_attacking_3() {
        let board = BoardState::state_from_string_fen(
            "rnbqk1nr/pppppppp/8/8/1b6/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let expected = 1 << Tables::B4;
        let result = board.black_attacking(&tables, 1 << Tables::E1);
    }

    #[test]
    fn black_attacking_4() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/1ppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let expected = 1 << Tables::A8;
        let result = board.black_attacking(&tables, 1 << Tables::A2);
        assert_eq!(expected, result);
    }

    #[test]
    fn black_attacking_5() {
        let board = BoardState::state_from_string_fen(
            "r1bqkbnr/pppppppp/8/8/8/6n1/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let expected = 1 << Tables::G3;
        let result = board.black_attacking(&tables, 1 << Tables::H1);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_safe_for_king_1() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let play = MoveRep::new(
            1 << Tables::E2,
            1 << Tables::E4,
            None,
            PieceType::Pawn,
            None,
        );
        let result = board.move_safe_for_king(&tables, &play);
        assert_eq!(true, result);
    }

    #[test]
    fn test_safe_for_king_2() {
        let mut board = BoardState::state_from_string_fen(
            "rn1qkbnr/p1pppppp/b7/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let play = MoveRep::new(
            1 << Tables::E1,
            1 << Tables::E2,
            None,
            PieceType::King,
            None,
        );
        let result = board.move_safe_for_king(&tables, &play);
        assert_eq!(false, result);
    }

    #[test]
    fn test_safe_for_king_3() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppp1ppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let play = MoveRep::new(
            1 << Tables::E8,
            1 << Tables::E7,
            None,
            PieceType::King,
            None,
        );
        let result = board.move_safe_for_king(&tables, &play);
        assert_eq!(true, result);
    }

    #[test]
    fn test_safe_for_king_4() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/Q7/8/PP1PPPPP/RNB1KBNR b KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let play = MoveRep::new(
            1 << Tables::D7,
            1 << Tables::D6,
            None,
            PieceType::Pawn,
            None,
        );
        let result = board.move_safe_for_king(&tables, &play);
        assert_eq!(false, result);
    }

    #[test]
    fn test_is_reversible_1() {
        let mv = MoveRep::new(
            1 << Tables::D2,
            1 << Tables::D4,
            None,
            PieceType::Pawn,
            None,
        );

        let result = mv.is_reversible();
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_reversible_2() {
        let mv = MoveRep::new(
            1 << Tables::B1,
            1 << Tables::A3,
            None,
            PieceType::Knight,
            None,
        );

        let result = mv.is_reversible();
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_reversible_3() {
        let mv = MoveRep::new(
            1 << Tables::C1,
            1 << Tables::G5,
            None,
            PieceType::Bishop,
            Some(PieceType::Pawn),
        );

        let result = mv.is_reversible();
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_reversible_4() {
        let mv = MoveRep::new(
            1 << Tables::E1,
            1 << Tables::A1,
            None,
            PieceType::King,
            None,
        );

        let result = mv.is_reversible();
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_reversible_5() {
        let mv = MoveRep::new(
            1 << Tables::G7,
            1 << Tables::G8,
            Some(Promotion::Queen),
            PieceType::Pawn,
            None,
        );

        let result = mv.is_reversible();
        assert_eq!(result, false);
    }

    #[test]
    fn test_en_passant_1() {
        let mut board = BoardState::starting_state();

        let mv_1 = MoveRep::new(
            1 << Tables::B2,
            1 << Tables::B4,
            None,
            PieceType::Pawn,
            None,
        );

        let mv_2 = MoveRep::new(
            1 << Tables::G8,
            1 << Tables::F6,
            None,
            PieceType::Knight,
            None,
        );

        assert_eq!(board.en_passant_target, 0);
        board.make(&mv_1);
        assert_eq!(board.en_passant_target, 1 << Tables::B3);
        board.make(&mv_2);
        assert_eq!(board.en_passant_target, 0);
        board.unmake(&mv_2);
        assert_eq!(board.en_passant_target, 1 << Tables::B3);
        board.unmake(&mv_1);
        assert_eq!(board.en_passant_target, 0);
    }

    #[test]
    fn test_en_passant_2() {
        let mut board = BoardState::starting_state();

        let mv_1 = MoveRep::new(
            1 << Tables::B1,
            1 << Tables::C3,
            None,
            PieceType::Knight,
            None,
        );

        board.make(&mv_1);
        assert_eq!(board.en_passant_target, 0);
        board.unmake(&mv_1);
        assert_eq!(board.en_passant_target, 0);
    }

    #[test]
    fn test_en_passant_3() {
        let mut board = BoardState::starting_state();

        let mv_1 = MoveRep::new(
            1 << Tables::B2,
            1 << Tables::B3,
            None,
            PieceType::Pawn,
            None,
        );

        board.make(&mv_1);
        assert_eq!(board.en_passant_target, 0);
    }

    #[test]
    fn test_en_passant_4() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b KQkq - 0 1".to_string(),
        );

        let mv_1 = MoveRep::new(
            1 << Tables::E7,
            1 << Tables::E5,
            None,
            PieceType::Pawn,
            None,
        );

        board.make(&mv_1);
        assert_eq!(board.en_passant_target, 1 << Tables::E6);
        board.unmake(&mv_1);
        assert_eq!(board.en_passant_target, 0);
    }

    #[test]
    fn test_en_passant_attack_1() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/ppp1pppp/8/8/2Pp4/8/PP1PPPPP/RNBQKBNR b KQkq c3 0 1".to_string(),
        );

        let tables = Tables::new();

        let expected_mv = MoveRep::new(
            1 << Tables::D4,
            1 << Tables::C3,
            None,
            PieceType::Pawn,
            Some(PieceType::Pawn),
        );

        let results = generate(&board, &tables);
        assert!(results.contains(&expected_mv));
    }

    #[test]
    fn test_en_passant_attack_2() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/ppp1pppp/8/2Pp4/8/8/PP1PPPPP/RNBQKBNR w KQkq d6 0 1".to_string(),
        );

        let tables = Tables::new();

        let expected_mv = MoveRep::new(
            1 << Tables::C5,
            1 << Tables::D6,
            None,
            PieceType::Pawn,
            Some(PieceType::Pawn),
        );

        let results = generate(&board, &tables);
        assert!(results.contains(&expected_mv));
    }

    #[test]
    fn test_en_passant_move_1() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/p1pppppp/8/Pp6/8/8/1PPPPPPP/RNBQKBNR w KQkq b6 0 1".to_string(),
        );

        let tables = Tables::new();

        let expected_mv = MoveRep::new(
            1 << Tables::A5,
            1 << Tables::B6,
            None,
            PieceType::Pawn,
            Some(PieceType::Pawn),
        );

        board.make(&expected_mv);
        print_bitboard(board.occupancy());
        assert_eq!(board.black_pawns, 0xbf000000000000);
        board.unmake(&expected_mv);
        assert_eq!(board.black_pawns, 0xbf004000000000);
    }

    #[test]
    fn test_pin_mask_1() {
        let board = BoardState::state_from_string_fen(
            "1nbqkbnr/pppppppp/8/4r3/8/8/PPPPQPPP/RNB1KBNR w - - 0 1".to_string(),
        );
        let tables = Tables::new();

        let target = 1 << Tables::E1;
        let expected = 1 << Tables::E2;
        let mask = board.pin_mask(&tables, target, board.white_to_move);
        assert_eq!(mask, expected);
    }

    #[test]
    fn test_pin_mask_2() {
        let board = BoardState::state_from_string_fen(
            "1nbqkbnr/pppppppp/8/4R3/8/8/PPPPQPPP/RNB1KBN1 b - - 0 1".to_string(),
        );
        let tables = Tables::new();

        let target = 1 << Tables::E8;
        let expected = 1 << Tables::E7;
        let mask = board.pin_mask(&tables, target, board.white_to_move);
        assert_eq!(mask, expected);
    }

    #[test]
    fn test_pin_mask_3() {
        let board = BoardState::state_from_string_fen(
            "rnb1kbnr/pp1ppppp/2p5/q7/8/2NP4/PPP1PPPP/R1BQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let target = 1 << Tables::E1;
        let expected = 1 << Tables::C3;
        let mask = board.pin_mask(&tables, target, board.white_to_move);
        assert_eq!(mask, expected);
    }

    #[test]
    fn test_pin_mask_4() {
        let board = BoardState::state_from_string_fen(
            "1nb1kbnr/pp1ppppp/2p1r3/q7/8/2NP4/PPP1PPPP/R1BQKBNR w - - 0 1".to_string(),
        );
        let tables = Tables::new();

        let target = 1 << Tables::E1;
        let expected = (1 << Tables::C3) | (1 << Tables::E2);
        let mask = board.pin_mask(&tables, target, board.white_to_move);
        assert_eq!(mask, expected);
    }

    #[test]
    fn test_pin_safe_1() {
        let board = BoardState::state_from_string_fen(
            "1nb1kbnr/pp1ppppp/2p1r3/q7/8/2NP4/PPP1PPPP/R1BQKBNR w - - 0 1".to_string(),
        );
        let tables = Tables::new();
        let target = 1 << Tables::E1;
        let mv = MoveRep::new(
            1 << Tables::G1,
            1 << Tables::H3,
            None,
            PieceType::Knight,
            None,
        );

        let result = board.pin_safe(&tables, target, &mv);
        assert_eq!(result, true);
    }

    #[test]
    fn test_pin_safe_2() {
        let board = BoardState::state_from_string_fen(
            "1nb1kbnr/pp1ppppp/2p1r3/q7/8/2NP4/PPP1PPPP/R1BQKBNR w - - 0 1".to_string(),
        );
        let tables = Tables::new();
        let target = 1 << Tables::E1;
        let mv = MoveRep::new(
            1 << Tables::E2,
            1 << Tables::E3,
            None,
            PieceType::Pawn,
            None,
        );

        let result = board.pin_safe(&tables, target, &mv);
        assert_eq!(result, true);
    }

    #[test]
    fn test_pin_safe_3() {
        let board = BoardState::state_from_string_fen(
            "1nb1kbnr/pp1ppppp/2p1r3/q7/8/2NP4/PPP1PPPP/R1BQKBNR w - - 0 1".to_string(),
        );
        let tables = Tables::new();
        let target = 1 << Tables::E1;
        let mv = MoveRep::new(
            1 << Tables::C3,
            1 << Tables::E4,
            None,
            PieceType::Knight,
            None,
        );

        let result = board.pin_safe(&tables, target, &mv);
        assert_eq!(result, false);
    }

    #[test]
    fn test_pin_safe_4() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/Q7/8/PP1PPPPP/RNB1KBNR b KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let target = 1 << Tables::E8;
        let mv = MoveRep::new(
            1 << Tables::D7,
            1 << Tables::D5,
            None,
            PieceType::Pawn,
            None,
        );

        let result = board.pin_safe(&tables, target, &mv);
        assert_eq!(result, false);
    }

    #[test]
    fn test_pin_safe_5() {
        let board = BoardState::state_from_string_fen(
            "rnbqkbnr/p1p1pppp/8/1p1p4/Q7/P1P5/1P1PPPPP/RNB1KBNR b KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let target = 1 << Tables::E8;
        let mv = MoveRep::new(
            1 << Tables::B5,
            1 << Tables::A4,
            None,
            PieceType::Pawn,
            Some(PieceType::Queen),
        );

        let result = board.pin_safe(&tables, target, &mv);
        assert_eq!(result, true);
    }

    #[test]
    fn test_white_kingside_castle() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/Q7/3BPN2/PP1P1PPP/RNB1K2R w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let mv = MoveRep::new(
            1 << Tables::E1,
            1 << Tables::G1,
            Some(Promotion::Castle),
            PieceType::King,
            None,
        );
        board.make(&mv);
        assert_eq!(board.white_king, 1 << Tables::G1);
        assert_eq!(board.white_rooks & 1 << Tables::F1, 1 << Tables::F1);
        assert_eq!(board.white_kingside_castle_rights, false);
        assert_eq!(board.white_queenside_castle_rights, false);

        board.unmake(&mv);
        assert_eq!(board.white_king, 1 << Tables::E1);
        assert_eq!(board.white_rooks & 1 << Tables::H1, 1 << Tables::H1);
        assert_eq!(board.white_kingside_castle_rights, true);
        assert_eq!(board.white_queenside_castle_rights, true);
    }
    #[test]
    fn test_white_queenside_castle() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/3P4/2NQB3/PPP1PPPP/R3KBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let mv = MoveRep::new(
            1 << Tables::E1,
            1 << Tables::C1,
            Some(Promotion::Castle),
            PieceType::King,
            None,
        );
        board.make(&mv);
        assert_eq!(board.white_king, 1 << Tables::C1);
        assert_eq!(board.white_rooks & 1 << Tables::D1, 1 << Tables::D1);
        assert_eq!(board.white_kingside_castle_rights, false);
        assert_eq!(board.white_queenside_castle_rights, false);

        board.unmake(&mv);
        assert_eq!(board.white_king, 1 << Tables::E1);
        assert_eq!(board.white_rooks & 1 << Tables::A1, 1 << Tables::A1);
        assert_eq!(board.white_kingside_castle_rights, true);
        assert_eq!(board.white_queenside_castle_rights, true);
    }
    #[test]
    fn test_black_kingside_castle() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqk2r/pppppp1p/5n1b/6p1/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let mv = MoveRep::new(
            1 << Tables::E8,
            1 << Tables::G8,
            Some(Promotion::Castle),
            PieceType::King,
            None,
        );
        board.make(&mv);
        assert_eq!(board.black_king, 1 << Tables::G8);
        assert_eq!(board.black_rooks & 1 << Tables::F8, 1 << Tables::F8);
        assert_eq!(board.black_kingside_castle_rights, false);
        assert_eq!(board.black_queenside_castle_rights, false);

        board.unmake(&mv);
        assert_eq!(board.black_king, 1 << Tables::E8);
        assert_eq!(board.black_rooks & 1 << Tables::H8, 1 << Tables::H8);
        assert_eq!(board.black_kingside_castle_rights, true);
        assert_eq!(board.black_queenside_castle_rights, true);
    }
    #[test]
    fn test_black_queenside_castle() {
        let mut board = BoardState::state_from_string_fen(
            "r3kbnr/ppp1pppp/2nqb3/3p4/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();
        let mv = MoveRep::new(
            1 << Tables::E8,
            1 << Tables::C8,
            Some(Promotion::Castle),
            PieceType::King,
            None,
        );
        board.make(&mv);
        assert_eq!(board.black_king, 1 << Tables::C8);
        assert_eq!(board.black_rooks & 1 << Tables::D8, 1 << Tables::D8);
        assert_eq!(board.black_kingside_castle_rights, false);
        assert_eq!(board.black_queenside_castle_rights, false);

        board.unmake(&mv);
        assert_eq!(board.black_king, 1 << Tables::E8);
        assert_eq!(board.black_rooks & 1 << Tables::A8, 1 << Tables::A8);
        assert_eq!(board.black_kingside_castle_rights, true);
        assert_eq!(board.black_queenside_castle_rights, true);
    }
    #[test]
    fn test_white_remove_kingside_castle_rights() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/7P/8/PPPPPPP1/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let kingside_rook_move = MoveRep::new(
            1 << Tables::H1,
            1 << Tables::H2,
            None,
            PieceType::Rook,
            None,
        );

        board.make(&kingside_rook_move);
        assert_eq!(board.white_kingside_castle_rights, false);
        assert_eq!(board.white_queenside_castle_rights, true);
    }
    #[test]
    fn test_white_remove_queenside_castle_right() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/P6P/8/1PPPPPP1/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let queenside_rook_move = MoveRep::new(
            1 << Tables::A1,
            1 << Tables::A2,
            None,
            PieceType::Rook,
            None,
        );

        board.make(&queenside_rook_move);
        assert_eq!(board.white_kingside_castle_rights, true);
        assert_eq!(board.white_queenside_castle_rights, false);
    }
    #[test]
    fn test_white_remove_all_castle_rights() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/pppppppp/8/8/P3P2P/8/1PPP1PP1/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let king_move = MoveRep::new(
            1 << Tables::E1,
            1 << Tables::E2,
            None,
            PieceType::King,
            None,
        );

        board.make(&king_move);
        assert_eq!(board.white_kingside_castle_rights, false);
        assert_eq!(board.white_queenside_castle_rights, false);
    }
    #[test]
    fn test_black_remove_kingside_castle_rights() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/ppppppp1/8/7p/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let kingside_rook_move = MoveRep::new(
            1 << Tables::H8,
            1 << Tables::H7,
            None,
            PieceType::Rook,
            None,
        );

        board.make(&kingside_rook_move);
        assert_eq!(board.black_kingside_castle_rights, false);
        assert_eq!(board.black_queenside_castle_rights, true);
    }
    #[test]
    fn test_black_remove_queenside_castle_right() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/1pppppp1/8/p6p/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let queenside_rook_move = MoveRep::new(
            1 << Tables::A8,
            1 << Tables::A7,
            None,
            PieceType::Rook,
            None,
        );

        board.make(&queenside_rook_move);
        assert_eq!(board.black_kingside_castle_rights, true);
        assert_eq!(board.black_queenside_castle_rights, false);
    }
    #[test]
    fn test_black_remove_all_castle_rights() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkbnr/1pppppp1/8/p6p/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let king_move = MoveRep::new(
            1 << Tables::E8,
            1 << Tables::E7,
            None,
            PieceType::King,
            None,
        );

        board.make(&king_move);
        assert_eq!(board.black_kingside_castle_rights, false);
        assert_eq!(board.black_queenside_castle_rights, false);
    }

    #[test]
    fn test_white_promotion() {
        let mut board = BoardState::state_from_string_fen(
            "rnbqkb2/pppppp1P/8/8/8/8/PPPPP1PP/RNBQKBNR w KQq - 0 1".to_string(),
        );
        let tables = Tables::new();

        let mv = MoveRep::new(
            1 << Tables::H7,
            1 << Tables::H8,
            Some(Promotion::Queen),
            PieceType::Pawn,
            None,
        );
        board.make(&mv);
        assert_eq!(board.white_queens, 1 << Tables::D1 | 1 << Tables::H8);
        assert_eq!(board.white_pawns & 1 << Tables::H8, 0);
    }
}
