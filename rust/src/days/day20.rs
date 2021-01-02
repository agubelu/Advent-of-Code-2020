use std::fs::read_to_string;
use std::time::Instant;

use rustc_hash::FxHashMap;

use crate::etc::d20::{Tile, BorderInfo, FullBorderInfo, Transformation, Rotation::*};
use crate::etc::utils::MatNum;

///////////////////////////////////////////////////////////////////////////////

type TileVec = Vec<Tile>;
type TileMap = FxHashMap<usize, Tile>;

type MatchLocations = [bool; 4];  // Top, right, bottom, left
type MatchMap = FxHashMap<usize, MatchInfo>;
struct MatchInfo {
    tiles: Vec<usize>,
    locations: MatchLocations,
    tile_borders: FullBorderInfo,
}

///////////////////////////////////////////////////////////////////////////////

pub fn run() {
    let time = Instant::now();
    let tiles = read_tiles();

    let n_tiles = tiles.len();
    let square_side = (n_tiles as f64).sqrt() as usize;

    let match_info = get_matches_info(&tiles);
    let corners: Vec<(&usize, &MatchInfo)> = match_info.iter()
                                                       .filter(|(_, m_info)| m_info.tiles.len() == 2)
                                                       .collect();

    let sol_part_1 = corners.iter().map(|(id, _)| *id).fold(1, |a, b| a * b);
    
    // Grab one corner and rotate it, so that the matching borders are on the right and bottom
    let initial_border = get_initial_border(corners[0], &tiles);
    
    // Note: I'm fairly sure this can be done in a slightly easier iterative way, since it seems that
    // each border in each tile only matches exactly another border in some other tile
    // However, this may be an specific oddity in my input, so we'll do it with backtracking
    // just in case. It shouldn't backtrack if I'm right anyways.
    let mut arranged_tiles = vec![initial_border];
    let res = arrange_tiles(&mut arranged_tiles, &tiles, &match_info, square_side);
    assert!(res, "Tile arrangement failed!");

    let big_tile = make_big_tile(&arranged_tiles, square_side);
    let sol_part_2 = find_spots_not_monster(big_tile);

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}

///////////////////////////////////////////////////////////////////////////////

fn find_spots_not_monster(mut tile: Tile) -> usize {
    let mut i = 0;
    let tile_side = tile.side();
    let wrap = tile_side - 18;
    // Positions in a slice in which to look for a monster
    let monster_pos = [0, wrap, wrap+5, wrap+6, wrap+11, wrap+12, wrap+17, wrap+18, wrap+19,
                       2*wrap+19, 2*wrap+22, 2*wrap+25, 2*wrap+28, 2*wrap+31, 2*wrap+34];
    let slice_size = 2*wrap + 35;

    loop {
        let flat = tile.flat();
        let n_monsters = (0..flat.len() - slice_size)
            .map(|i| &flat[i..i+slice_size])
            .filter(|slice| monster_pos.iter().map(|&pos| slice[pos]).sum::<u8>() == 15)
            .count();

        if n_monsters > 1 {
            return flat.iter().filter(|x| **x == 1).count() - n_monsters * 15;
        }

        i += 1;
        tile = tile.transform((Rot90, i == 4));
    }
}

fn make_big_tile(tiles: &TileVec, square_side: usize) -> Tile {
    let trimmed_side = tiles[0].side() - 2;
    let big_tile_side = (trimmed_side) * square_side;
    let mut big_mat = MatNum::new(big_tile_side, big_tile_side, 0);

    for (i, tile) in tiles.iter().enumerate() {
        let offset_x = (i % square_side) * trimmed_side;
        let offset_y = (i / square_side) * trimmed_side;

        for y in 0..trimmed_side {
            for x in 0..trimmed_side {
                big_mat[(offset_x + x, offset_y + y)] = tile.mat().get(x+1, y+1);
            }
        }
    }

    return Tile::from_mat(0, big_mat);
}

fn arrange_tiles(arrangement: &mut Vec<Tile>, tiles: &TileMap, match_info: &MatchMap, side: usize) -> bool {
    let it = arrangement.len();
    
    if it == tiles.len() {
        return true;
    }

    let first_col = it % side == 0;
    let neighbor_ind = if first_col {it - side} else {it - 1};
    let previous_tile = &arrangement[neighbor_ind];

    let candidate_tile_ids: Vec<&usize> = match_info[&previous_tile.id()].tiles.iter()
        .filter(|&ind| !arrangement.iter().any(|tile| tile.id() == *ind))
        .collect();

    let previous_borders = previous_tile.border_info();

    for cand_id in candidate_tile_ids {
        let cand_borders = &match_info[cand_id].tile_borders;
        let trans = find_trans(cand_borders, &previous_borders, first_col);

        if trans.is_none() {
            continue; 
        }

        let next_tile = tiles[cand_id].transform(trans.unwrap());
        arrangement.push(next_tile);

        let res = arrange_tiles(arrangement, tiles, match_info, side);

        if !res {
            // We hit a dead end, remove the tile we just inserted
            arrangement.pop();
        } else {
            return true;
        }
    }

    return false;
}

fn find_trans(cand_borders: &FullBorderInfo, previous_borders: &BorderInfo, is_first_col: bool) -> Option<Transformation> {
    let border_to_match = previous_borders[if is_first_col {2} else {1}];

    for (i, &border) in cand_borders.iter().enumerate() {
        if border != border_to_match { continue; }

        // We have a match. Now, determine the exact transformation that is needed
        // so that the proposed tile matches the previous one.
        // Lots of cases, depending on which border we matched, whether it was flipped,
        // if it's the first column or not...
        // Maybe there's a way to generalize this but I don't feel like finding it rn
        let transf = match (i, is_first_col) {
            (0, false) => (Rot270, true),
            (1, false) => (Rot0, true),
            (2, false) => (Rot270, false),
            (3, false) => (Rot0, false),
            (4, false) => (Rot90, false),
            (5, false) => (Rot180, false),
            (6, false) => (Rot90, true),
            (7, false) => (Rot180, true),
            (0,  true) => (Rot0, false),
            (1,  true) => (Rot90, false),
            (2,  true) => (Rot180, true),
            (3,  true) => (Rot270, true),
            (4,  true) => (Rot0, true),
            (5,  true) => (Rot90, true),
            (6,  true) => (Rot180, false),
            (7,  true) => (Rot270, false),
             _ => panic!("wtf")
        };

        return Some(transf);
    }

    return None;
}

fn get_initial_border(corner: (&usize, &MatchInfo), tiles: &TileMap) -> Tile {
    let orig_tile = &tiles[corner.0];

    // Get the necessary transformation to ensure that the matching borders
    // are on the right and bottom
    let transf = match corner.1.locations {
        [false, true, true, false] => (Rot0, false),
        [false, false, true, true] => (Rot90, false),
        [true, false, false, true] => (Rot180, false),
        [true, true, false, false] => (Rot270, false),
        _ => panic!("wtf"),
    };

    return orig_tile.transform(transf);
}

fn get_matches_info(tiles: &TileMap) -> MatchMap {
    let borders: Vec<(usize, FullBorderInfo)> = tiles.iter().map(|(&id, tile)| (id, tile.full_border_info())).collect();
 
    borders.iter().map(|(id, my_borders)| {
        let mut loc_info = [false; 4];
        let mut matching_ids = Vec::with_capacity(4);
        
        for i in 0..4 {
            let my_border = my_borders[i];

            for (other_id, other_borders) in &borders {
                if id == other_id { continue; }

                for &other_border in other_borders {
                    if other_border == my_border {
                        matching_ids.push(*other_id);
                        loc_info[i] = true;
                        break;
                    }
                }

                if matching_ids.len() == 4 { break; }
            }
        }

        (*id, MatchInfo{tiles: matching_ids, locations: loc_info, tile_borders: *my_borders})
    }).collect()
}

fn read_tiles() -> TileMap {
    let data = read_to_string("../input/day20.txt").expect("Error reading file");

    data.split("\n\n").map(|tile_data| {
        let tile_lines: Vec<&str> = tile_data.split("\n").collect();
        let len_0 = tile_lines[0].len();
        let mat_size = tile_lines[1].len();

        let tile_id = tile_lines[0][5..len_0-1].parse().unwrap();
        let mut mat = MatNum::new(mat_size, mat_size, 0);

        for y in 0..mat_size {
            for (x, ch) in tile_lines[y + 1].chars().enumerate() {
                if ch == '#' {
                    mat.set(x, y, 1);
                }
            }
        }

        (tile_id, Tile::from_mat(tile_id, mat))
    }).collect()
}