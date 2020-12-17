from etc.utils import file_to_lines
from itertools import product, chain

lines = file_to_lines("input/day17.txt")

def solve_for_dim(n_dims):
    DIFFS = list(product((1, 0, -1), repeat=n_dims))

    cur_state = {}

    for y, line in enumerate(lines):
        for x, val in enumerate(line):
            if val == "#":
                cur_state[(x, y) + (0,) * (n_dims - 2)] = 1
    
    get_neighbors = lambda pos: [tuple([pos[i] + diff[i] for i in range(n_dims)]) for diff in DIFFS]
    count_nearby = lambda pos, state: sum(state.get(neigh, 0) for neigh in get_neighbors(pos) if neigh != pos)

    def get_next(pos, state):
        current = state.get(pos, 0)
        count = count_nearby(pos, state)

        if current == 1 and count not in (2, 3):
            return 0
        elif current == 0 and count == 3:
            return 1
        else:
            return current

    for i in range(6):
        next_state = {}
        pos_analyze = set(chain.from_iterable(get_neighbors(pos) for pos in cur_state))
        for pos in pos_analyze:
            next_val = get_next(pos, cur_state)
            if next_val:
                next_state[pos] = next_val

        cur_state = next_state

    return len(cur_state)

print("Part 1:", solve_for_dim(3))
print("Part 2:", solve_for_dim(4))
