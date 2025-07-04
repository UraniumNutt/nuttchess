use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

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
            0x0880001080204000,
            0x0040004010002003,
            0x2a00201082004008,
            0x0600040940201200,
            0x0a00084c62005020,
            0x0e00040810820011,
            0x0c80008021000200,
            0x01000c8041000032,
            0x0000800080204008,
            0x0401802000400082,
            0x2002002842008010,
            0x001100081001a302,
            0x0446000a00041020,
            0x0802000804020010,
            0x3362004108020044,
            0x0902000410920041,
            0x80008a8002400220,
            0x0700404010002004,
            0x2401010020004010,
            0x0c20210008100100,
            0x0004008080080004,
            0x2001010002080400,
            0x4890a40032011810,
            0x0640120000510084,
            0x0040208080004000,
            0x2520004140003000,
            0x4000200280100088,
            0x823c100080080280,
            0x0060050100100800,
            0x2052000a00100834,
            0x0824010400489022,
            0x026028820000490c,
            0x0020804010800020,
            0x0000200081804000,
            0x0020006081801000,
            0x0010100101002008,
            0x0004004008080080,
            0x2411002803000400,
            0x4040080104001002,
            0x210051004a000084,
            0x0080102000484004,
            0x1000500020004002,
            0x8020420480220012,
            0x8800100409010020,
            0x0048008004008008,
            0x0802000804020010,
            0x3362004108020044,
            0x4800804081220004,
            0x0000400080002080,
            0x7280409022010200,
            0x0001004020081100,
            0x8540880280100480,
            0x0100080080040280,
            0x0000020004008080,
            0x1100014810024400,
            0x2e51000882004900,
            0x1041490150220082,
            0x0042102100804003,
            0x2800408020081202,
            0x00d0000810210105,
            0x82020008102004aa,
            0x000100084c000229,
            0x10081710080a9204,
            0x0824010400489022,
        ];

        let bishop_magics: [u64; 64] = [
            0x3e02280111020200,
            0x3e02280111020200,
            0x0026082a00820a10,
            0x22080605c0100041,
            0x4204152001005800,
            0x0a408820086002a8,
            0x3000888410421000,
            0x0020804108202281,
            0x010ad02002040068,
            0x0041083000808904,
            0x2080100122002000,
            0x2080040506030407,
            0x2000021210000000,
            0x9108021104200090,
            0x0400004104202101,
            0x0802042488280848,
            0x9044202084140800,
            0x2050000450108101,
            0x0201000802040010,
            0x8414045803a84c04,
            0x0041010820081000,
            0x008a000422012022,
            0x0110820400841100,
            0x0001020480881180,
            0x1d20b00160040920,
            0x0001080004284804,
            0x020910080c404200,
            0x6014080080202040,
            0x1801010040104000,
            0x1000808108080410,
            0x0001040040440405,
            0x2061020000524c01,
            0x9418080a0040c210,
            0x0082101000030200,
            0x0041004100080808,
            0x9800400820060200,
            0x0020062081040084,
            0x2000a10100020080,
            0x0008420090704810,
            0x2000a10100020080,
            0x4001101822082480,
            0x0204088208811000,
            0x8000501090001800,
            0x4200022104002042,
            0x00001c3008840400,
            0x206000a100404200,
            0x2804080204200040,
            0x0224008212040050,
            0x3000888410421000,
            0x3000888410421000,
            0x2010020100882211,
            0x0000000084041280,
            0x0000002082440009,
            0x30181004a8082850,
            0xc042080244004000,
            0x3e02280111020200,
            0x0020804108202281,
            0x0802042488280848,
            0x0080004301084903,
            0x00001c3008840400,
            0x9a020c4040050100,
            0x1404800504080204,
            0x010ad02002040068,
            0x3e02280111020200,
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
        Tables::generate_bishop_attacks(
            &mut bishop_attacks,
            relevent_bishop_count,
            bishop_magics,
            bishop_occupancy,
        );

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

    pub fn get_rook_attack(&self, square: usize, occupancy: u64) -> u64 {
        let magic = self.rook_magics[square];
        let hash = Tables::apply_magic_hash(
            magic,
            self.relevent_rook_count[square],
            occupancy & self.rook_occupancy[square],
        );
        self.rook_attacks[hash as usize][square]
    }
    pub fn get_bishop_attack(&self, square: usize, occupancy: u64) -> u64 {
        let magic = self.bishop_magics[square];
        let hash = Tables::apply_magic_hash(
            magic,
            self.relevent_bishop_count[square],
            occupancy & self.bishop_occupancy[square],
        );
        self.bishop_attacks[hash as usize][square]
    }

    fn generate_white_pawn_attacks(table: &mut [u64; 64]) {
        for shift_value in 0..64 {
            let mask = 1 << shift_value;
            // If the pawn is on the last rank, dont do anything
            if mask & Self::RANK_8 != 0 {
                table[shift_value] = 0;
                continue;
            }
            // Otherwise, the normal attack patterns
            if mask & Self::FILE_A == 0 {
                table[shift_value] |= 1 << (shift_value + 9);
            }
            if mask & Self::FILE_H == 0 {
                table[shift_value] |= 1 << (shift_value + 7);
            }
        }
    }

    fn generate_black_pawn_attacks(table: &mut [u64; 64]) {
        for shift_value in 0..64 {
            let mask = 1 << shift_value;
            // If the pawn is on the first rank, dont do anything
            if mask & Self::RANK_1 != 0 {
                table[shift_value] = 0;
                continue;
            }
            // Otherwise, the normal attack patterns
            if mask & Self::FILE_A == 0 {
                table[shift_value] |= 1 << (shift_value - 7);
            }
            if mask & Self::FILE_H == 0 {
                table[shift_value] |= 1 << (shift_value - 9);
            }
        }
    }

    fn generate_white_pawn_push(table: &mut [u64; 64]) {
        for shift_value in 0..64 {
            let mask = 1 << shift_value;
            // If the pawn is on the last rank (or somehow on the first rank?), it can't be pushed
            if mask & Self::RANK_8 != 0 || mask & Self::RANK_1 != 0 {
                table[shift_value] = 0;
                continue;
            }
            // If the pawn is on the second / starting rank, add the double push
            if mask & Self::RANK_2 != 0 {
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
            if mask & Self::RANK_8 != 0 || mask & Self::RANK_1 != 0 {
                table[shift_value] = 0;
                continue;
            }
            // If the pawn is on the seventh / black starting rank, add the double push
            if mask & Self::RANK_7 != 0 {
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
            if mask & Self::RANK_8 == 0 {
                table[shift_value] |= 1 << (shift_value + 8);
            }
            // North east
            if mask & Self::RANK_8 == 0 && mask & Self::FILE_H == 0 {
                table[shift_value] |= 1 << (shift_value + 7);
            }
            // East
            if mask & Self::FILE_H == 0 {
                table[shift_value] |= 1 << (shift_value - 1);
            }
            // South east
            if mask & Self::RANK_1 == 0 && mask & Self::FILE_H == 0 {
                table[shift_value] |= 1 << (shift_value - 9);
            }
            // South
            if mask & Self::RANK_1 == 0 {
                table[shift_value] |= 1 << (shift_value - 8);
            }
            // South west
            if mask & Self::RANK_1 == 0 && mask & Self::FILE_A == 0 {
                table[shift_value] |= 1 << (shift_value - 7);
            }
            // West
            if mask & Self::FILE_A == 0 {
                table[shift_value] |= 1 << (shift_value + 1);
            }
            // North west
            if mask & Self::FILE_A == 0 && mask & Self::RANK_8 == 0 {
                table[shift_value] |= 1 << (shift_value + 9);
            }
        }
    }

    fn generate_knight_attacks(table: &mut [u64; 64]) {
        for shift_value in 0..64 {
            let mask = 1 << shift_value;

            // North north east
            if mask & Self::RANK_78 == 0 && mask & Self::FILE_H == 0 {
                table[shift_value] |= 1 << (shift_value + 15);
            }
            // North east east
            if mask & Self::RANK_8 == 0 && mask & Self::FILE_GH == 0 {
                table[shift_value] |= 1 << (shift_value + 6);
            }
            // South east east
            if mask & Self::RANK_1 == 0 && mask & Self::FILE_GH == 0 {
                table[shift_value] |= 1 << (shift_value - 10);
            }
            // South south east
            if mask & Self::RANK_12 == 0 && mask & Self::FILE_H == 0 {
                table[shift_value] |= 1 << (shift_value - 17);
            }
            // South south west
            if mask & Self::RANK_12 == 0 && mask & Self::FILE_A == 0 {
                table[shift_value] |= 1 << (shift_value - 15);
            }
            // South west west
            if mask & Self::RANK_1 == 0 && mask & Self::FILE_AB == 0 {
                table[shift_value] |= 1 << (shift_value - 6);
            }
            // North west west
            if mask & Self::RANK_8 == 0 && mask & Self::FILE_AB == 0 {
                table[shift_value] |= 1 << (shift_value + 10);
            }
            // North north west
            if mask & Self::RANK_78 == 0 && mask & Self::FILE_A == 0 {
                table[shift_value] |= 1 << (shift_value + 17);
            }
        }
    }

    // Get the occupancy mask for the rooks
    pub fn generate_rook_occupancy_mask(table: &mut [u64; 64]) {
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
        let mut loop_rank = rank;
        while loop_rank < 7 {
            loop_rank += 1;
            let mask = 1 << Tables::rf_to_index(loop_rank, file);
            if mask & blockers != 0 {
                relevent |= mask;
                break;
            }
            relevent |= mask;
        }
        // East
        let mut loop_file = file;
        while loop_file > 0 {
            loop_file -= 1;
            let mask = 1 << Tables::rf_to_index(rank, loop_file);
            if mask & blockers != 0 {
                relevent |= mask;
                break;
            }
            relevent |= mask;
        }
        // South
        let mut loop_rank = rank;
        while loop_rank > 0 {
            loop_rank -= 1;
            let mask = 1 << Tables::rf_to_index(loop_rank, file);
            if mask & blockers != 0 {
                relevent |= mask;
                break;
            }
            relevent |= mask;
        }
        // West
        let mut loop_file = file;
        while loop_file < 7 {
            loop_file += 1;
            let mask = 1 << Tables::rf_to_index(rank, loop_file);
            if mask & blockers != 0 {
                relevent |= mask;
                break;
            }
            relevent |= mask;
        }
        relevent
    }

    // Get the occupancy mask for the bishops
    pub fn generate_bishop_occupancy_mask(table: &mut [u64; 64]) {
        for shift_value in 0..64 {
            let rank = shift_value / 8;
            let file = shift_value % 8;
            let mask = 1 << shift_value;

            // North east
            if mask & Self::FILE_H == 0 {
                let mut rank_loop = rank + 1;
                let mut file_loop = file - 1;
                while rank_loop < 7 && file_loop > 0 {
                    table[shift_value as usize] |= 1 << Tables::rf_to_index(rank_loop, file_loop);
                    rank_loop += 1;
                    file_loop -= 1;
                }
            }
            // South east
            if mask & Self::FILE_H == 0 && mask & Self::RANK_1 == 0 {
                let mut rank_loop = rank - 1;
                let mut file_loop = file - 1;
                while rank_loop > 0 && file_loop > 0 {
                    table[shift_value as usize] |= 1 << Tables::rf_to_index(rank_loop, file_loop);
                    rank_loop -= 1;
                    file_loop -= 1;
                }
            }
            // South west
            if mask & Self::RANK_1 == 0 {
                let mut rank_loop = rank - 1;
                let mut file_loop = file + 1;
                while rank_loop > 0 && file_loop < 7 {
                    table[shift_value as usize] |= 1 << Tables::rf_to_index(rank_loop, file_loop);
                    rank_loop -= 1;
                    file_loop += 1;
                }
            }
            // North west
            let mut rank_loop = rank + 1;
            let mut file_loop = file + 1;
            while rank_loop < 7 && file_loop < 7 {
                table[shift_value as usize] |= 1 << Tables::rf_to_index(rank_loop, file_loop);
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
    pub fn calculate_relevent_bishop_occupancy(index: usize, blockers: u64) -> u64 {
        let rank = index / 8;
        let file = index % 8;
        let mut relevent = 0;

        // North east
        let mut rank_loop = rank;
        let mut file_loop = file;
        while rank_loop < 7 && file_loop > 0 {
            rank_loop += 1;
            file_loop -= 1;
            let loop_mask = 1 << Tables::rf_to_index(rank_loop, file_loop);
            if loop_mask & blockers != 0 {
                relevent |= loop_mask;
                break;
            }
            relevent |= loop_mask;
        }
        // South east
        let mut rank_loop = rank;
        let mut file_loop = file;
        while rank_loop > 0 && file_loop > 0 {
            rank_loop -= 1;
            file_loop -= 1;
            let loop_mask = 1 << Tables::rf_to_index(rank_loop, file_loop);
            if loop_mask & blockers != 0 {
                relevent |= loop_mask;
                break;
            }
            relevent |= loop_mask;
        }
        // South west
        let mut rank_loop = rank;
        let mut file_loop = file;
        while rank_loop > 0 && file_loop < 7 {
            rank_loop -= 1;
            file_loop += 1;
            let loop_mask = 1 << Tables::rf_to_index(rank_loop, file_loop);
            if loop_mask & blockers != 0 {
                relevent |= loop_mask;
                break;
            }
            relevent |= loop_mask;
        }
        // North west
        let mut rank_loop = rank;
        let mut file_loop = file;
        while rank_loop < 7 && file_loop < 7 {
            rank_loop += 1;
            file_loop += 1;
            let loop_mask = 1 << Tables::rf_to_index(rank_loop, file_loop);
            if loop_mask & blockers != 0 {
                relevent |= loop_mask;
                break;
            }
            relevent |= loop_mask;
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
                tables[hash as usize][square] = Tables::calculate_relevent_rook_occupancy(
                    square,
                    Tables::map_number_to_occupancy(permutation, occupancy[square]),
                );
            }
        }
    }
    fn generate_bishop_attacks(
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
                tables[hash as usize][square] = Tables::calculate_relevent_bishop_occupancy(
                    square,
                    Tables::map_number_to_occupancy(permutation, occupancy[square]),
                );
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
        let mut found_magic;

        // Stores the relevent occupancies generated
        let mut relevent_occupancies = [0; 4096];
        loop {
            // Start to apply the hash to see if it works
            let magic = Tables::get_possible_magic(&mut rng);
            if ((magic.wrapping_mul(attack_mask)) & 0xFF00000000000000).count_ones() < 6 {
                continue;
            }
            relevent_occupancies.fill(0);
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

    // If we ever need to regenerate some magics:
    // let mut rook_magics = [0; 64];
    // let mut bishop_magics = [0; 64];
    // let mut rook_mask = [0; 64];
    // let mut bishop_mask = [0; 64];
    // Tables::generate_rook_occupancy_mask(&mut rook_mask);
    // Tables::generate_bishop_occupancy_mask(&mut bishop_mask);
    // for square in 0..64 {
    //     rook_magics[square] = Tables::generate_magic(
    //         rook_mask[square],
    //         square,
    //         rook_mask[square].count_ones() as u64,
    //         &Tables::calculate_relevent_rook_occupancy,
    //     );
    //     bishop_magics[square] = Tables::generate_magic(
    //         bishop_mask[square],
    //         square,
    //         bishop_mask[square].count_ones() as u64,
    //         &Tables::calculate_relevent_bishop_occupancy,
    //     );
    // }

    // println!("Rook magics: ");
    // for square in 0..64 {
    //     println!("{:#018x}", rook_magics[square]);
    // }

    // println!("Bishop magics: ");
    // for square in 0..64 {
    //     println!("{:#018x}", bishop_magics[square]);
    // }

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

#[cfg(test)]
mod tests {
    use crate::{generate::generate, print_bitboard, BoardState};

    use super::*;

    #[test]
    fn test_calculate_relevent_rook_occupancy_1() {
        let test_occupancy1 = 0x4080000000008080;
        let result1 = Tables::calculate_relevent_rook_occupancy(63, test_occupancy1);
        assert_eq!(result1, 0x4080000000000000);
    }

    #[test]
    fn test_calculate_relevent_rook_occupancy_2() {
        let test_occupancy2 = 0x800040;
        let result2 = Tables::calculate_relevent_rook_occupancy(7, test_occupancy2);
        assert_eq!(result2, 0x808040);
    }

    #[test]
    fn test_calculate_relevent_rook_occupancy_3() {
        let test_occupancy3 = 8388735;
        let result3 = Tables::calculate_relevent_rook_occupancy(7, test_occupancy3);
        assert_eq!(result3, 0x808040);
    }

    #[test]
    fn test_calculate_relevent_rook_occupancy_4() {
        let test_occupancy = 0xffff;
        let result = Tables::calculate_relevent_rook_occupancy(7, test_occupancy);
        assert_eq!(result, 0x8040);
    }

    #[test]
    fn test_calculate_relevent_bishop_occupancy() {
        let test1 = 0x82000000084400;
        let expected1 = 0x82442800284000;
        let result1 = Tables::calculate_relevent_bishop_occupancy(28, test1);
        println!("Input: {}", test1);
        print_bitboard(test1);
        println!("Expected: {}", expected1);
        print_bitboard(expected1);
        println!("Actual: {}", result1);
        print_bitboard(result1);
        assert_eq!(result1, expected1);
    }

    #[test]
    fn test_calculate_relevent_bishop_occupancy_2() {
        let test2 = 0x40010000000;
        let expected2 = 0x2040000000000;
        let result2 = Tables::calculate_relevent_bishop_occupancy(56, test2);
        println!("Input: {}", test2);
        print_bitboard(test2);
        println!("Expected: {}", expected2);
        print_bitboard(expected2);
        println!("Actual: {}", result2);
        print_bitboard(result2);
        assert_eq!(result2, expected2);
    }

    #[test]
    fn test_calculate_relevent_bishop_occupancy_3() {
        let test3 = 0x8000001000080;
        let expected3 = 0x10a000a11204080;
        let result3 = Tables::calculate_relevent_bishop_occupancy(42, test3);
        println!("Input: {}", test3);
        print_bitboard(test3);
        println!("Expected: {}", expected3);
        print_bitboard(expected3);
        println!("Actual: {}", result3);
        print_bitboard(result3);
        assert_eq!(result3, expected3);
    }

    #[test]
    fn test_calculate_relevent_bishop_occupancy_4() {
        let test = 0x100000000000000;
        let expected = 0x102040810a000a0;
        let result = Tables::calculate_relevent_bishop_occupancy(14, test);
        assert_eq!(result, expected);
    }
}
