from etc.utils import file_to_lines
from etc.d20 import Tile

from math import sqrt, prod
from itertools import product
import regex as re

tiles_raw = file_to_lines("input/day20.txt", spl="\n\n")
tiles = list(map(Tile.from_string, tiles_raw))
n_tiles = len(tiles)
square_side = int(sqrt(n_tiles))

tile_mods = list(product((False, True), (False, True), (0, 1, 2, 3)))

# Find the pieces that belong to the corners (Part 1), and also use one of those
# as the starting point for the backtracking algorithm

def get_borders_with_matches(tile, ls):
    other_tiles = [t for t in ls if t.tile_id != tile.tile_id]
    borders = tile.get_borders()
    res = tuple()

    for i, border in enumerate(borders):
        for other_tile in other_tiles:
            if any(border == ob for ob in other_tile.get_borders_and_invs()):
                res += (i,)

    return res

corners = [(t, get_borders_with_matches(t, tiles)) for t in tiles]
corners = list(filter(lambda x: len(x[1]) == 2, corners))
sol_part_1 = prod(t[0].tile_id for t in corners)
print("Part 1:", sol_part_1)

# Now, grab one of the corners and rotate it so that the borders that
# match some other border are 1 (right) and 2 (bottom)
# That will be our top-left starting tile
corner_data = corners[0]

# I'm 90% sure this has to match one of these 4 options
starting_tile = {
    (0, 1): corner_data[0].transform(False, False, 1),
    (0, 3): corner_data[0].transform(False, False, 2),
    (1, 2): corner_data[0],
    (2, 3): corner_data[0].transform(False, False, 3),
}[corner_data[1]]

def find_placements(current_tiles, used_tiles, i):
    available_tiles = [tile for tile in tiles if tile.tile_id not in used_tiles]

    if not available_tiles:
        return current_tiles

    for tile in available_tiles:
        for mod in tile_mods:
            proposed_tile = tile.transform(*mod)
            if is_compat(current_tiles, proposed_tile, i):
                res = find_placements(current_tiles + [proposed_tile], used_tiles + [tile.tile_id], i + 1)
                if res is not None: return res

def is_compat(current_tiles, proposed_tile: Tile, i):
    row, col = i // square_side, i % square_side
    prop_borders = proposed_tile.get_borders()

    left_match = col == 0 or current_tiles[i - 1].get_borders()[1] == prop_borders[3]
    top_match = row == 0 or current_tiles[i - square_side].get_borders()[2] == prop_borders[0]
    return left_match and top_match

# Find the group of tiles that compose the full image, and trim the borders
composed_tiles = find_placements([starting_tile], [starting_tile.tile_id], 1)
trimmed_contents = [t.get_trimmed_content() for t in composed_tiles]
len_trimmed = len(trimmed_contents[0])  # It's still a square

# Create the big tile
big_tile_content = []
for i in range(square_side):
    for row_n in range(len_trimmed):
        big_tile_content.append("".join(trimmed_contents[ind][row_n] for ind in range(square_side * i, square_side * (i + 1))))
        
big_tile = Tile(0, big_tile_content)
pic_width = len(big_tile.content[0])

# This regex matches a monster when the picture is flattened into a single line
wrap = pic_width - 18
re_monster = re.compile(r"#.{%d}#(.{4}##){3}#.{%d}(#.{2}){6}" % (wrap - 1, wrap - 1))

# Positions in which a # belongs to a monster relative to the start of
# the monster in the flattened matrix
monster_pos_offset = (0, wrap, 5, 1, 5, 1, 5, 1, 1, wrap, 3, 3, 3, 3, 3)
sol_part_2 = None

# Let's go monster hunting
for mod in tile_mods:
    flat = big_tile.transform(*mod).flatten()
    monsters = list(re_monster.finditer(flat, overlapped=True))

    if len(monsters) > 1:
        # Allow up to one match by mere chance (it doesn't happen for my input, but just in case)
        monster_positions = set()

        for match in monsters:
            start_pos = match.start()
            for i in range(1, len(monster_pos_offset) + 1):
                monster_positions.add(start_pos + sum(monster_pos_offset[:i]))
        
        sol_part_2 = flat.count("#") - len(monster_positions)
        break

print("Part 2:", sol_part_2)
