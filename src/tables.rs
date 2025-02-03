use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::print_bitboard;

// constants for board indicies
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
    pub rook_magics: [u64; 64],
    pub bishop_magics: [u64; 64],
    pub rook_attacks: Vec<Vec<u64>>,
    pub bishop_attacks: Vec<Vec<u64>>,
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
        let mut rook_attacks: Vec<Vec<u64>> = vec![vec![0; 64]; 4096];
        let mut bishop_attacks: Vec<Vec<u64>> = vec![vec![0; 64]; 4096];

        let rook_magics: [u64; 64] = [
            0x880001080204000,
            0x40004010002003,
            0x2a00201082004008,
            0x600040940201200,
            0xa00084c62005020,
            0xe00040810820011,
            0xc80008021000200,
            0x1000c8041000032,
            0x00800080204008,
            0x401802000400082,
            0x2002002842008010,
            0x1100081001a302,
            0x446000a00041020,
            0x802000804020010,
            0x3362004108020044,
            0x902000410920041,
            0x80008a8002400220,
            0x700404010002004,
            0x2401010020004010,
            0xc20210008100100,
            0x04008080080004,
            0x2001010002080400,
            0x4890a40032011810,
            0x640120000510084,
            0x40208080004000,
            0x2520004140003000,
            0x4000200280100088,
            0x823c100080080280,
            0x60050100100800,
            0x2052000a00100834,
            0x824010400489022,
            0x26028820000490c,
            0x20804010800020,
            0x00200081804000,
            0x20006081801000,
            0x10100101002008,
            0x04004008080080,
            0x2411002803000400,
            0x4040080104001002,
            0x210051004a000084,
            0x80102000484004,
            0x1000500020004002,
            0x8020420480220012,
            0x8800100409010020,
            0x48008004008008,
            0x802000804020010,
            0x3362004108020044,
            0x4800804081220004,
            0x00400080002080,
            0x7280409022010200,
            0x01004020081100,
            0x8540880280100480,
            0x100080080040280,
            0x00020004008080,
            0x1100014810024400,
            0x2e51000882004900,
            0x1041490150220082,
            0x42102100804003,
            0x2800408020081202,
            0xd0000810210105,
            0x82020008102004aa,
            0x0100084c000229,
            0x10081710080a9204,
            0x824010400489022,
        ];

        let bishop_magics: [u64; 64] = [
            0x204200d0c102008,
            0x20a00820d1880000,
            0x26082a00820a10,
            0x22080605c0100041,
            0x4204152001005800,
            0xa408820086002a8,
            0x08a60508008504,
            0x068a2050040000,
            0x10ad02002040068,
            0x41083000808904,
            0x2080100122002000,
            0x2080040506030407,
            0x2000021210000000,
            0x9108021104200090,
            0x80040c0104108010,
            0x0a108284090002,
            0x9044202084140800,
            0x2050000450108101,
            0x201000802040010,
            0x8414045803a84c04,
            0x41010820081000,
            0x8a000422012022,
            0x110820400841100,
            0x01020480881180,
            0x1d20b00160040920,
            0x01080004284804,
            0x20910080c404200,
            0x6014080080202040,
            0x1801010040104000,
            0x1000808108080410,
            0x01040040440405,
            0x2061020000524c01,
            0x9418080a0040c210,
            0x82101000030200,
            0x41004100080808,
            0x9800400820060200,
            0x20062081040084,
            0x2000a10100020080,
            0x08420090704810,
            0x2000a10100020080,
            0x4001101822082480,
            0x204088208811000,
            0x8000501090001800,
            0x4200022104002042,
            0x001c3008840400,
            0x206000a100404200,
            0x2804080204200040,
            0x224008212040050,
            0x500b105a100010,
            0x800701c5240000,
            0x2010020100882211,
            0x00000084041280,
            0x00002082440009,
            0x2000021021821200,
            0x800701c5240000,
            0x8000122408a4500,
            0x1000002210040402,
            0x00000084041280,
            0x5600040004012390,
            0x001c3008840400,
            0x9a020c4040050100,
            0x1404800504080204,
            0x2320002310410b00,
            0x2001225002049102,
        ];

        // Do the generation logic here
        Tables::generate_white_pawn_push(&mut white_pawn_push);
        Tables::generate_black_pawn_push(&mut black_pawn_push);
        Tables::generate_white_pawn_attacks(&mut white_pawn_attacks);
        Tables::generate_black_pawn_attacks(&mut black_pawn_attacks);
        Tables::generate_king_attacks(&mut king_attacks);
        Tables::generate_knight_attacks(&mut knight_attacks);
        Tables::generate_rook_occupancy_mask(&mut rook_occupancy);
        Tables::generate_bishop_occupancy_mask(&mut bishop_occupancy);
        Tables::generate_queen_occupancy_mask(&mut queen_occupancy);
        Tables::generate_count_table(&mut relevent_rook_count, rook_occupancy);
        Tables::generate_count_table(&mut relevent_bishop_count, bishop_occupancy);
        Tables::generate_rook_attacks(
            &mut rook_attacks,
            relevent_rook_count,
            rook_magics,
            rook_occupancy,
        );
        Tables::generate_bishop_attacks(&mut bishop_attacks, relevent_bishop_count, bishop_magics);

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
            rook_magics,
            bishop_magics,
            rook_attacks,
            bishop_attacks,
        }
    }

    pub fn get_rook_attack(self, square: usize, mask: u64) -> u64 {
        let magic = self.rook_magics[square];
        let hash = Tables::apply_magic_hash(magic, self.relevent_rook_count[square], mask);
        self.rook_attacks[hash as usize][square]
    }
    pub fn get_bishop_attack(self, square: usize, mask: u64) -> u64 {
        let magic = self.bishop_magics[square];
        let hash = Tables::apply_magic_hash(magic, self.relevent_bishop_count[square], mask);
        self.bishop_attacks[hash as usize][square]
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
        let mut loop_rank = rank + 1;
        while loop_rank < 7 {
            let mask = 1 << Tables::rf_to_index(loop_rank, file);
            if mask & blockers != 0 {
                relevent |= mask;
                break;
            }
            loop_rank += 1;
        }
        // East
        let mut loop_file = 1;
        while loop_file < file {
            let mask = 1 << Tables::rf_to_index(rank, loop_file);
            if mask & blockers != 0 {
                relevent |= mask;
                break;
            }
            loop_file += 1;
        }
        // South
        let mut loop_rank = 1;
        while loop_rank < rank {
            let mask = 1 << Tables::rf_to_index(loop_rank, file);
            if mask & blockers != 0 {
                relevent |= mask;
                break;
            }
            loop_rank += 1;
        }
        // West
        let mut loop_file = file + 1;
        while loop_file < 7 {
            let mask = 1 << Tables::rf_to_index(rank, loop_file);
            if mask & blockers != 0 {
                relevent |= mask;
                break;
            }
            loop_file += 1;
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

    fn generate_queen_occupancy_mask(table: &mut [u64; 64]) {
        Tables::generate_rook_occupancy_mask(table);
        Tables::generate_bishop_occupancy_mask(table);
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

    fn generate_rook_attacks(
        tables: &mut Vec<Vec<u64>>,
        count: [u64; 64],
        magics: [u64; 64],
        occupancy: [u64; 64],
    ) {
        for square in 0..64 {
            for permutation in 0..(1 << count[square]) {
                let hash = Tables::apply_magic_hash(
                    magics[square],
                    count[square],
                    Tables::map_number_to_occupancy(permutation, occupancy[square]),
                );
                tables[hash as usize][square] =
                    Tables::calculate_relevent_rook_occupancy(square, permutation);
            }
        }
    }
    fn generate_bishop_attacks(tables: &mut Vec<Vec<u64>>, count: [u64; 64], magics: [u64; 64]) {
        for square in 0..64 {
            for permutation in 0..(1 << count[square]) {
                let hash = Tables::apply_magic_hash(magics[square], count[square], permutation);
                tables[hash as usize][square] =
                    Tables::calculate_relevent_bishops_occupancy(square, permutation);
            }
        }
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

    // Generate a magic number: will only use this make pregenerated tables
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
