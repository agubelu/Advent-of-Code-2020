use std::io::{BufReader, BufRead, Error};
use std::fs::File;
use std::time::Instant;

use rustc_hash::FxHashSet;
use counter::Counter;

///////////////////////////////////////////////////////////////////////////////

type Coords = (i32, i32);
type MoveList = Vec<Coords>;
type CoordsSet = FxHashSet<Coords>;

//                            E       SE        SW        W        NW      NE
const MOVES: [Coords; 6] = [(2, 0), (1, -2), (-1, -2), (-2, 0), (-1, 2), (1, 2)];

///////////////////////////////////////////////////////////////////////////////

pub fn run() {
    let time = Instant::now();

    let f = BufReader::new(File::open("../input/day24.txt").unwrap());
    let movelist = read_moves(f);

    let mut black_tiles = FxHashSet::default();

    for moves in movelist {
        let mut coords = (0, 0);
        for mv in moves {
            coords = (coords.0 + mv.0, coords.1 + mv.1);
        }

        if !black_tiles.contains(&coords) {
            black_tiles.insert(coords);
        } else {
            black_tiles.remove(&coords);
        }
    }
    
    let sol_part_1 = black_tiles.len();

    for _ in 0..100 {
        let adj_count = black_tiles.iter().flat_map(get_adj).collect::<Counter<_>>();
        let mut stay_black: CoordsSet = black_tiles.iter()
                                                   .filter(|t| match adj_count.get(&t) {
                                                       None => false,
                                                       Some(v) => *v <= 2,
                                                   })
                                                   .map(|k| *k)
                                                   .collect();

        let flip_black: CoordsSet = adj_count.iter()
                                             .filter(|(k, v)| !black_tiles.contains(&k) && **v == 2)
                                             .map(|(k, _)| *k)
                                             .collect();

        stay_black.extend(flip_black);
        black_tiles = stay_black;
    }

    let sol_part_2 = black_tiles.len();

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}

///////////////////////////////////////////////////////////////////////////////

fn get_adj(coords: &Coords) -> Vec<Coords> {
    return MOVES.iter().map(|(x, y)| (x + coords.0, y + coords.1)).collect();
}

fn read_moves(f: BufReader<File>) -> Vec<MoveList> {
    return f.lines().map(line_to_moves).collect();
}

fn line_to_moves(line: Result<String, Error>) -> MoveList {
    let mut res = Vec::new();
    let line = line.unwrap();
    let mut chars = line.chars();

    while let Some(ch) = chars.next() {
        let mv = match ch {
            'e' => 0,
            'w' => 3,
            's' => 1 + (chars.next().unwrap() == 'w') as usize,
            'n' => 4 + (chars.next().unwrap() == 'e') as usize,
            _ => panic!(),
        };
        res.push(MOVES[mv]);
    }

    return res;
}