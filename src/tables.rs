// constants for board indicies

use std::process::Output;

use rand::seq::index;
use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::print_bitboard;

// rank 1
pub const A1: u64 = 7;
pub const B1: u64 = 6;
pub const C1: u64 = 5;
pub const D1: u64 = 4;
pub const E1: u64 = 3;
pub const F1: u64 = 2;
pub const G1: u64 = 1;
pub const H1: u64 = 0;
// rank 2
pub const A2: u64 = 15;
pub const B2: u64 = 14;
pub const C2: u64 = 13;
pub const D2: u64 = 12;
pub const E2: u64 = 11;
pub const F2: u64 = 10;
pub const G2: u64 = 9;
pub const H2: u64 = 8;
// rank 3
pub const A3: u64 = 23;
pub const B3: u64 = 22;
pub const C3: u64 = 21;
pub const D3: u64 = 20;
pub const E3: u64 = 19;
pub const F3: u64 = 18;
pub const G3: u64 = 17;
pub const H3: u64 = 16;
// rank 4
pub const A4: u64 = 31;
pub const B4: u64 = 30;
pub const C4: u64 = 29;
pub const D4: u64 = 28;
pub const E4: u64 = 27;
pub const F4: u64 = 26;
pub const G4: u64 = 25;
pub const H4: u64 = 24;
// rank 5
pub const A5: u64 = 39;
pub const B5: u64 = 38;
pub const C5: u64 = 37;
pub const D5: u64 = 36;
pub const E5: u64 = 35;
pub const F5: u64 = 34;
pub const G5: u64 = 33;
pub const H5: u64 = 32;
// rank 6
pub const A6: u64 = 47;
pub const B6: u64 = 46;
pub const C6: u64 = 45;
pub const D6: u64 = 44;
pub const E6: u64 = 43;
pub const F6: u64 = 42;
pub const G6: u64 = 41;
pub const H6: u64 = 40;
// rank 7
pub const A7: u64 = 55;
pub const B7: u64 = 54;
pub const C7: u64 = 53;
pub const D7: u64 = 52;
pub const E7: u64 = 51;
pub const F7: u64 = 50;
pub const G7: u64 = 49;
pub const H7: u64 = 48;
// rank 8
pub const A8: u64 = 63;
pub const B8: u64 = 62;
pub const C8: u64 = 61;
pub const D8: u64 = 60;
pub const E8: u64 = 59;
pub const F8: u64 = 58;
pub const G8: u64 = 57;
pub const H8: u64 = 56;

// Consts for rank / file masks
// Files
pub const FILE_A: u64 = 0x8080808080808080;
pub const FILE_B: u64 = 0x4040404040404040;
pub const FILE_C: u64 = 0x2020202020202020;
pub const FILE_D: u64 = 0x1010101010101010;
pub const FILE_E: u64 = 0x808080808080808;
pub const FILE_F: u64 = 0x404040404040404;
pub const FILE_G: u64 = 0x202020202020202;
pub const FILE_H: u64 = 0x101010101010101;
// Ranks
pub const RANK_1: u64 = 0xff;
pub const RANK_2: u64 = 0xff00;
pub const RANK_3: u64 = 0xff0000;
pub const RANK_4: u64 = 0xff000000;
pub const RANK_5: u64 = 0xff00000000;
pub const RANK_6: u64 = 0xff0000000000;
pub const RANK_7: u64 = 0xff000000000000;
pub const RANK_8: u64 = 0xff00000000000000;
// Double rank / files (usefull for knights)
pub const FILE_AB: u64 = 0xc0c0c0c0c0c0c0c0;
pub const FILE_GH: u64 = 0x303030303030303;

pub const RANK_12: u64 = 0xffff;
pub const RANK_78: u64 = 0xffff000000000000;

pub struct Tables {
    // Look up table for attack bitboards
    pub white_pawn_attacks: [u64; 64],
    pub black_pawn_attacks: [u64; 64],
    pub white_pawn_push: [u64; 64],
    pub black_pawn_push: [u64; 64],
    pub knight_attacks: [u64; 64],
    pub rook_occupancy: [u64; 64],
    pub bishop_occupancy: [u64; 64],
    pub queen_occupancy: [u64; 64],
    pub king_attacks: [u64; 64],
    pub relevent_rook_count: [u64; 64],
    pub relevent_bishop_count: [u64; 64],
}
impl Tables {
    pub fn new() -> Tables {
        // Init the tables
        let mut white_pawn_attacks: [u64; 64] = [0; 64];
        let mut black_pawn_attacks: [u64; 64] = [0; 64];
        let mut white_pawn_push: [u64; 64] = [0; 64];
        let mut black_pawn_push: [u64; 64] = [0; 64];
        let mut knight_attacks: [u64; 64] = [0; 64];
        let mut rook_occupancy: [u64; 64] = [0; 64];
        let mut bishop_occupancy: [u64; 64] = [0; 64];
        let mut queen_occupancy: [u64; 64] = [0; 64];
        let mut king_attacks: [u64; 64] = [0; 64];
        let mut relevent_rook_count: [u64; 64] = [0; 64];
        let mut relevent_bishop_count: [u64; 64] = [0; 64];

        // Do the generation logic here
        Tables::generate_white_pawn_push(&mut white_pawn_push);
        Tables::generate_black_pawn_push(&mut black_pawn_push);
        Tables::generate_white_pawn_attacks(&mut white_pawn_attacks);
        Tables::generate_black_pawn_attacks(&mut black_pawn_attacks);
        Tables::generate_king_attacks(&mut king_attacks);
        Tables::generate_knight_attacks(&mut knight_attacks);
        Tables::generate_rook_occupancy_mask(&mut rook_occupancy);
        Tables::generate_bishop_occupancy_mask(&mut bishop_occupancy);
        Tables::generate_count_table(&mut relevent_rook_count, rook_occupancy);
        Tables::generate_count_table(&mut relevent_bishop_count, bishop_occupancy);

        // Now return the struct
        Tables {
            white_pawn_attacks,
            black_pawn_attacks,
            white_pawn_push,
            black_pawn_push,
            knight_attacks,
            rook_occupancy,
            bishop_occupancy,
            queen_occupancy,
            king_attacks,
            relevent_rook_count,
            relevent_bishop_count,
        }
    }

    fn generate_white_pawn_attacks(table: &mut [u64; 64]) {
        for shift_value in 0..64 {
            let mask = 1 << shift_value;
            // If the pawn is on the first or last ranks dont do anything
            if mask & RANK_8 != 0 || mask & RANK_1 != 0 {
                table[shift_value] = 0;
                continue;
            }
            // Otherwise, the normal attack patterns
            if mask & FILE_A == 0 {
                table[shift_value] |= 1 << (shift_value + 9);
            }
            if mask & FILE_H == 0 {
                table[shift_value] |= 1 << (shift_value + 7);
            }
        }
    }

    fn generate_black_pawn_attacks(table: &mut [u64; 64]) {
        for shift_value in 0..64 {
            let mask = 1 << shift_value;
            // If the pawn is on the first or last ranks dont do anything
            if mask & RANK_8 != 0 || mask & RANK_1 != 0 {
                table[shift_value] = 0;
                continue;
            }
            // Otherwise, the normal attack patterns
            if mask & FILE_A == 0 {
                table[shift_value] |= 1 << (shift_value - 7);
            }
            if mask & FILE_H == 0 {
                table[shift_value] |= 1 << (shift_value - 9);
            }
        }
    }
    fn generate_white_pawn_push(table: &mut [u64; 64]) {
        for shift_value in 0..64 {
            let mask = 1 << shift_value;
            // If the pawn is on the last rank (or somehow on the first rank?), it can't be pushed
            if mask & RANK_8 != 0 || mask & RANK_1 != 0 {
                table[shift_value] = 0;
                continue;
            }
            // If the pawn is on the second / starting rank, add the double push
            if mask & RANK_2 != 0 {
                table[shift_value] |= 1 << (shift_value + 16);
            }
            // In other cases, add the single push
            table[shift_value] |= 1 << (shift_value + 8);
        }
    }

    fn generate_black_pawn_push(table: &mut [u64; 64]) {
        for shift_value in 0..64 {
            let mask = 1 << shift_value;
            // If the pawn is on the first rank (or somehow on the last rank?), it can't be pushed
            if mask & RANK_8 != 0 || mask & RANK_1 != 0 {
                table[shift_value] = 0;
                continue;
            }
            // If the pawn is on the seventh / black starting rank, add the double push
            if mask & RANK_7 != 0 {
                table[shift_value] |= 1 << (shift_value - 16);
            }
            // In other cases, add the single push
            table[shift_value] |= 1 << (shift_value - 8);
        }
    }

    fn generate_king_attacks(table: &mut [u64; 64]) {
        for shift_value in 0..64 {
            let mask = 1 << shift_value;

            // North
            if mask & RANK_8 == 0 {
                table[shift_value] |= 1 << (shift_value + 8);
            }
            // North east
            if mask & RANK_8 == 0 && mask & FILE_H == 0 {
                table[shift_value] |= 1 << (shift_value + 7);
            }
            // East
            if mask & FILE_H == 0 {
                table[shift_value] |= 1 << (shift_value - 1);
            }
            // South east
            if mask & RANK_1 == 0 && mask & FILE_H == 0 {
                table[shift_value] |= 1 << (shift_value - 9);
            }
            // South
            if mask & RANK_1 == 0 {
                table[shift_value] |= 1 << (shift_value - 8);
            }
            // South west
            if mask & RANK_1 == 0 && mask & FILE_A == 0 {
                table[shift_value] |= 1 << (shift_value - 7);
            }
            // West
            if mask & FILE_A == 0 {
                table[shift_value] |= 1 << (shift_value + 1);
            }
            // North west
            if mask & FILE_A == 0 && mask & RANK_8 == 0 {
                table[shift_value] |= 1 << (shift_value + 9);
            }
        }
    }

    fn generate_knight_attacks(table: &mut [u64; 64]) {
        for shift_value in 0..64 {
            let mask = 1 << shift_value;

            // North north east
            if mask & RANK_78 == 0 && mask & FILE_H == 0 {
                table[shift_value] |= 1 << (shift_value + 15);
            }
            // North east east
            if mask & RANK_8 == 0 && mask & FILE_GH == 0 {
                table[shift_value] |= 1 << (shift_value + 6);
            }
            // South east east
            if mask & RANK_1 == 0 && mask & FILE_GH == 0 {
                table[shift_value] |= 1 << (shift_value - 10);
            }
            // South south east
            if mask & RANK_12 == 0 && mask & FILE_H == 0 {
                table[shift_value] |= 1 << (shift_value - 17);
            }
            // South south west
            if mask & RANK_12 == 0 && mask & FILE_A == 0 {
                table[shift_value] |= 1 << (shift_value - 15);
            }
            // South west west
            if mask & RANK_1 == 0 && mask & FILE_AB == 0 {
                table[shift_value] |= 1 << (shift_value - 6);
            }
            // North west west
            if mask & RANK_8 == 0 && mask & FILE_AB == 0 {
                table[shift_value] |= 1 << (shift_value + 10);
            }
            // North north west
            if mask & RANK_78 == 0 && mask & FILE_A == 0 {
                table[shift_value] |= 1 << (shift_value + 17);
            }
        }
    }

    // Get the occupancy mask for the rooks
    fn generate_rook_occupancy_mask(table: &mut [u64; 64]) {
        for shift_value in 0..64 {
            let rank = shift_value / 8; // the number
            let file = shift_value % 8; // the letter

            // North
            for loop_rank in (rank + 1)..7 {
                table[shift_value] |= 1 << Tables::rf_to_index(loop_rank, file);
            }
            // East
            for loop_file in 1..file {
                table[shift_value] |= 1 << Tables::rf_to_index(rank, loop_file);
            }
            // South
            for loop_rank in 1..rank {
                table[shift_value] |= 1 << Tables::rf_to_index(loop_rank, file);
            }
            // West
            for loop_file in (file + 1)..7 {
                table[shift_value] |= 1 << Tables::rf_to_index(rank, loop_file);
            }
        }
    }

    // Calculate the relevent occupancy mask for the rooks
    pub fn calculate_relevent_rook_occupancy(index: usize, blockers: u64) -> u64 {
        let rank = index / 8;
        let file = index % 8;
        let mut relevent = 0;

        // North
        for loop_rank in (rank + 1)..7 {
            let mask = 1 << Tables::rf_to_index(loop_rank, file);
            if mask & blockers != 0 {
                relevent |= mask;
                break;
            }
        }
        // East
        for loop_file in 1..file {
            let mask = 1 << Tables::rf_to_index(rank, loop_file);
            if mask & blockers != 0 {
                relevent |= mask;
                break;
            }
        }
        // South
        for loop_rank in 1..rank {
            let mask = 1 << Tables::rf_to_index(loop_rank, file);
            if mask & blockers != 0 {
                relevent |= mask;
                break;
            }
        }
        // West
        for loop_file in (file + 1)..7 {
            let mask = 1 << Tables::rf_to_index(rank, loop_file);
            if mask & blockers != 0 {
                relevent |= mask;
                break;
            }
        }
        relevent
    }

    // Get the occupancy mask for the bishops
    fn generate_bishop_occupancy_mask(table: &mut [u64; 64]) {
        for shift_value in 0..64 {
            let rank = shift_value / 8;
            let file = shift_value % 8;
            let mask = 1 << shift_value;

            // North east
            if mask & FILE_H == 0 {
                let mut rank_loop = rank + 1;
                let mut file_loop = file - 1;
                while rank_loop < 7 && file_loop > 0 {
                    table[shift_value] |= 1 << Tables::rf_to_index(rank_loop, file_loop);
                    rank_loop += 1;
                    file_loop -= 1;
                }
            }
            // South east
            if mask & FILE_H == 0 && mask & RANK_1 == 0 {
                let mut rank_loop = rank - 1;
                let mut file_loop = file - 1;
                while rank_loop > 0 && file_loop > 0 {
                    table[shift_value] |= 1 << Tables::rf_to_index(rank_loop, file_loop);
                    rank_loop -= 1;
                    file_loop -= 1;
                }
            }
            // South west
            if mask & RANK_1 == 0 {
                let mut rank_loop = rank - 1;
                let mut file_loop = file + 1;
                while rank_loop > 0 && file_loop < 7 {
                    table[shift_value] |= 1 << Tables::rf_to_index(rank_loop, file_loop);
                    rank_loop -= 1;
                    file_loop += 1;
                }
            }
            // North west
            let mut rank_loop = rank + 1;
            let mut file_loop = file + 1;
            while rank_loop < 7 && file_loop < 7 {
                table[shift_value] |= 1 << Tables::rf_to_index(rank_loop, file_loop);
                rank_loop += 1;
                file_loop += 1;
            }
        }
    }

    // Calculate the relevent occupancy mask for the bishops
    pub fn calculate_relevent_bishops_occupancy(index: usize, blockers: u64) -> u64 {
        let rank = index / 8;
        let file = index % 8;
        let mask = 1 << index;
        let mut relevent = 0;

        // North east
        if mask & FILE_H == 0 {
            let mut rank_loop = rank + 1;
            let mut file_loop = file - 1;
            while rank_loop < 7 && file_loop > 0 {
                let loop_mask = 1 << Tables::rf_to_index(rank_loop, file_loop);
                if loop_mask & blockers != 0 {
                    relevent |= loop_mask;
                    break;
                }
                rank_loop += 1;
                file_loop -= 1;
            }
        }
        // South east
        if mask & FILE_H == 0 && mask & RANK_1 == 0 {
            let mut rank_loop = rank - 1;
            let mut file_loop = file - 1;
            while rank_loop > 0 && file_loop > 0 {
                let loop_mask = 1 << Tables::rf_to_index(rank_loop, file_loop);
                if loop_mask & blockers != 0 {
                    relevent |= loop_mask;
                    break;
                }
                rank_loop -= 1;
                file_loop -= 1;
            }
        }
        // South west
        if mask & RANK_1 == 0 {
            let mut rank_loop = rank - 1;
            let mut file_loop = file + 1;
            while rank_loop > 0 && file_loop < 7 {
                let loop_mask = 1 << Tables::rf_to_index(rank_loop, file_loop);
                if loop_mask & blockers != 0 {
                    relevent |= loop_mask;
                    break;
                }
                rank_loop -= 1;
                file_loop += 1;
            }
        }
        // North west
        let mut rank_loop = rank + 1;
        let mut file_loop = file + 1;
        while rank_loop < 7 && file_loop < 7 {
            let loop_mask = 1 << Tables::rf_to_index(rank_loop, file_loop);
            if loop_mask & blockers != 0 {
                relevent |= loop_mask;
                break;
            }
            rank_loop += 1;
            file_loop += 1;
        }

        relevent
    }

    // Generate a table of the counts based on the bit count of the masks
    fn generate_count_table(table: &mut [u64; 64], occupancy_masks: [u64; 64]) {
        for shift_value in 0..64 {
            table[shift_value] = occupancy_masks[shift_value].count_ones() as u64;
        }
    }

    // Sets the bits of the mask to the bits of the number from least to most significant
    pub fn map_number_to_occupancy(mut number: u64, occupancy: u64) -> u64 {
        let mut mapped = 0;

        for shift_value in 0..64 {
            if occupancy & (1 << shift_value) != 0 {
                let bit = match number & 1 {
                    0 => 0,
                    _ => 1,
                };
                number >>= 1;
                mapped |= bit << shift_value;
            }
        }

        mapped
    }

    // Generate a magic number
    pub fn generate_magic(
        attack_mask: u64,
        square_index: usize,
        bit_count: u64,
        calculate_relevent: &dyn Fn(usize, u64) -> u64,
    ) -> u64 {
        // let bit_count = attack_mask.count_ones() as u64;
        // The number of permutations of occupancies
        let attack_permutations = 1 << bit_count;
        // The permutations of occupancies
        let mut occupancies = [0; 4096];

        // Init the occupancies
        for index in 0..attack_permutations {
            occupancies[index] = Tables::map_number_to_occupancy(index as u64, attack_mask);
        }

        // Make a xorshift
        let mut rng = XorShiftRng::seed_from_u64(0);
        // Run until we get a good magic
        let mut found_magic = false;

        loop {
            // Start to apply the hash to see if it works
            let magic = Tables::get_possible_magic(&mut rng);
            if ((magic.wrapping_mul(attack_mask)) & 0xFF00000000000000).count_ones() < 6 {
                continue;
            }
            // Stores the relevent occupancies generated
            let mut relevent_occupancies = [0; 4096];
            found_magic = true;
            for index in 0..attack_permutations {
                // Apply the hash
                let hash = Tables::apply_magic_hash(magic, bit_count, occupancies[index]);
                let relevent_blockers = calculate_relevent(square_index, occupancies[index]);
                // If the relevent occupancies at the hash is zero, set it to the calculated occupancy
                if relevent_occupancies[hash as usize] == 0 {
                    relevent_occupancies[hash as usize] = relevent_blockers;
                // If it is not zero, then it has to contain the same relevent occupancies
                } else if relevent_occupancies[hash as usize] != relevent_blockers {
                    found_magic = false;
                    break;
                }
            }
            // If true, then the magic works
            if found_magic {
                return magic;
            }
        }
    }

    fn apply_magic_hash(magic: u64, bit_count: u64, mask: u64) -> u64 {
        mask.wrapping_mul(magic) >> (64 - bit_count)
    }

    fn get_random_u64(rng: &mut XorShiftRng) -> u64 {
        rng.next_u64()
    }

    // Get a number that is likely to be a magic
    pub fn get_possible_magic(rng: &mut XorShiftRng) -> u64 {
        Tables::get_random_u64(rng) & Tables::get_random_u64(rng) & Tables::get_random_u64(rng)
    }

    // Maps the rank and file to the index
    const fn rf_to_index(rank: usize, file: usize) -> u64 {
        (file + 8 * rank) as u64
    }

    // Gets the index of the least significant 1
    pub fn get_index(bb: u64) -> i64 {
        if bb == 0 {
            return -1;
        }

        (bb & !bb + 1).ilog2() as i64
    }
}
