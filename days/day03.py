from etc.utils import file_to_lines
from math import prod

lines = file_to_lines("input/day03.txt")
trees = {(x, y): char == "#" for y, line in enumerate(lines) for x, char in enumerate(line)}

line_len = len(lines[0])
n_lines = len(lines)

def get_num_trees(slope):
    x = y = 0
    n_trees = 0
    while y < n_lines:
        if trees[x % line_len, y]:
            n_trees += 1
        x += slope[0]
        y += slope[1]
    return n_trees

# Part 1
num_trees = get_num_trees((3, 1))
print("Part 1:", num_trees)

# Part 2
slopes = [(1, 1), (5, 1), (7, 1), (1, 2)]
mult = num_trees * prod(get_num_trees(slope) for slope in slopes)
print("Part 2:", mult)