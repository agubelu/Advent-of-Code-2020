from etc.utils import file_to_lines
from collections import Counter
from itertools import chain

MOVES = {
    "e": (+1, 0),
    "w": (-1, 0),
    "ne": (+0.5, +1),
    "nw": (-0.5, +1),
    "se": (+0.5, -1),
    "sw": (-0.5, -1)
}

def line_to_moves(line):
    moves = []
    while line:
        move_len = 1 if line[0] in ("e", "w") else 2
        moves.append(line[:move_len])
        line = line[move_len:]
    return moves

def get_adj(coords):
    return [(coords[0] + move[0], coords[1] + move[1]) for move in MOVES.values()]

moves_list = file_to_lines("input/day24.txt", func=line_to_moves)
tile_colors = {}  # False = white, True = black

for moves in moves_list:
    coords = (0, 0)
    for move in moves:
        diff = MOVES[move]
        coords = (coords[0] + diff[0], coords[1] + diff[1])
    
    tile_colors[coords] = not tile_colors.get(coords, False)

print("Part 1:", sum(map(int, tile_colors.values())))

for _ in range(100):
    black_tiles = set(t for t, v in tile_colors.items() if v)
    counter = Counter(chain.from_iterable(get_adj(t) for t in black_tiles))
    flip_white = [t for t in black_tiles if t not in counter or counter[t] > 2]
    flip_black = [t for t, v in counter.items() if t not in black_tiles and v == 2]

    for t in flip_white:
        tile_colors[t] = False

    for t in flip_black:
        tile_colors[t] = True

print("Part 2:", sum(map(int, tile_colors.values())))
