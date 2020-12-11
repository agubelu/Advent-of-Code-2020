from etc.utils import file_to_lines

DIRS = [(-1, -1), (0, -1), (+1, -1),
        (-1,  0),          (+1,  0),
        (-1, +1), (0, +1), (+1, +1)]

lines = file_to_lines("input/day11.txt")
max_x = len(lines[0])
max_y = len(lines)

# Preamble
def process_input(lines):
    coords_map = {}
    ls = []
    inserted = 0

    for y, line in enumerate(lines):
        for x, ch in enumerate(line):
            if ch == "L":
                ls.append(0)
                coords_map[x, y] = inserted
                inserted += 1

    adj1 = {i: find_adj_1(coords, i, coords_map) for coords, i in coords_map.items()}
    adj2 = {i: find_adj_2(coords, i, coords_map) for coords, i in coords_map.items()}

    return ls, adj1, adj2

def find_adj_1(coords, i, coords_map):
    res = []
    for dx, dy in DIRS:
        x = coords[0] + dx
        y = coords[1] + dy

        if 0 <= x < max_x and 0 <= y < max_y and (x, y) in coords_map:
            res.append(coords_map[x, y])
    return res

def find_adj_2(coords, i, coords_map):
    res = []
    for dx, dy in DIRS:
        x = coords[0] + dx
        y = coords[1] + dy

        while 0 <= x < max_x and 0 <= y < max_y:
            if (x, y) in coords_map:
                res.append(coords_map[x, y])
                break
            x += dx
            y += dy
    return res

def solve(ls, adj_map, max_around):
    def get_next(elem, i, cur_state):
        count = sum(cur_state[j] for j in adj_map[i])
        if elem == 0 and count == 0:
            return 1
        elif elem == 1 and count >= max_around:
            return 0
        else:
            return elem

    state = ls.copy()
    next_state = [get_next(k, i, state) for i, k in enumerate(state)]
    while state != next_state:
        state = next_state
        next_state = [get_next(k, i, state) for i, k in enumerate(state)]
    return sum(next_state)

# Magic
seats, adj1, adj2 = process_input(lines)
print("Part 1:", solve(seats, adj1, 4))
print("Part 2:", solve(seats, adj2, 5))
