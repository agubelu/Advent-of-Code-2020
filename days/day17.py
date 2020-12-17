from etc.utils import file_to_lines
from itertools import product, chain
from collections import Counter

lines = file_to_lines("input/day17.txt")

def solve_for_dim(n_dims):
    diffs = list(product((1, 0, -1), repeat=n_dims))
    diffs.remove((0,) * n_dims)

    cur_state = set()

    for y, line in enumerate(lines):
        for x, val in enumerate(line):
            if val == "#":
                cur_state.add((x, y) + (0,) * (n_dims - 2))
    
    get_neighbors = lambda pos: [tuple([pos[i] + diff[i] for i in range(n_dims)]) for diff in diffs]

    for i in range(6):
        counter = Counter(chain.from_iterable(get_neighbors(pos) for pos in cur_state))
        stay_on = set(x for x in cur_state if counter[x] in (2, 3))
        become_on = set(x for x, cnt in counter.items() if cnt == 3 and x not in cur_state)
        cur_state = stay_on.union(become_on)

    return len(cur_state)

print("Part 1:", solve_for_dim(3))
print("Part 2:", solve_for_dim(4))
