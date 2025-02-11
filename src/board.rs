use std::{
    future::Pending,
    io::{self, Write},
    process::Output,
};

#[derive(Clone)]
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
    pub fullmove_counter: u16,
    pub move_stack: Vec<MoveStackFrame>,
    pub move_stack_pointer: usize,
}

#[derive(Debug)]
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

#[derive(Copy, Clone, Debug)]
pub enum Promotion {
    Queen,
    Bishop,
    Rook,
    Knight,
}

// Helps the move maker know what bitboard to manipulate
#[derive(Copy, Clone, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

// Stores state of the board which can not be recovered when unmaking a move
#[derive(Copy, Clone)]
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
            fullmove_counter: 0,
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
                state.fullmove_counter = full_move_clock_int;
            } else {
                return Err("Error parsing the number of fullmoves".to_string());
            }
        } else {
            return Err("String does not have enough tokens to be a valid fen string".to_string());
        }

        Ok(state)
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

        let start = position_to_mask(char1, char2).unwrap();
        let end = position_to_mask(char3, char4).unwrap();
        let move_rep = self.move_rep_from_masks(start, end);
        self.make(&move_rep);
    }

    // Changes the board state to reflect the move. Also pushes to the move stack
    pub fn make(&mut self, play: &MoveRep) {
        self.clear(play.starting_square, Some(play.moved_type));
        self.clear_all(play.ending_square);
        self.set(play.ending_square, Some(play.moved_type));
        // Do special logic here
        self.white_to_move = !self.white_to_move;
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

    // Reverts the move from the board. Pops from the move stack
    pub fn unmake(&mut self, play: &MoveRep) {
        // Something is going wrong in unmake
        // it only happens when attacked is none
        let first_count = self.occupancy().count_ones();
        self.white_to_move = !self.white_to_move;
        // I guess swapping the order of the next two lines fixed it?
        self.set(play.ending_square, play.attacked_type);
        self.clear(play.ending_square, Some(play.moved_type));
        self.set(play.starting_square, Some(play.moved_type));
        let second_count = self.occupancy().count_ones();
        if play.attacked_type.is_none() {
            if second_count != first_count {
                let piece_string = match play.moved_type {
                    PieceType::Pawn => "pawn",
                    PieceType::Knight => "knight",

                    _ => "",
                };
                let color = match self.white_to_move {
                    true => "white",
                    false => "black",
                };
                let attacked = match play.attacked_type {
                    Some(_) => "some",
                    None => "none",
                };
                // print_bitboard(self.occupancy());
                // println!("{}", piece_string);
                // println!("{}", color);
                // println!("{}", attacked);
                // panic!();
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
        if self.white_bishops & mask != 0 || self.white_bishops & mask != 0 {
            return Some(PieceType::Bishop);
        }
        if self.white_rooks & mask != 0 || self.white_rooks & mask != 0 {
            return Some(PieceType::Rook);
        }
        if self.white_queens & mask != 0 || self.white_queens & mask != 0 {
            return Some(PieceType::Queen);
        }
        if self.white_king & mask != 0 || self.white_king & mask != 0 {
            return Some(PieceType::King);
        }

        None
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
