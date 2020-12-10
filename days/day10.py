from etc.utils import file_to_lines
import numpy as np

lines = [0] + file_to_lines("input/day10.txt", func=int)
n_lines = len(lines)
lines.sort()

# Part 1
diffs = {1: 0, 2: 0, 3: 1}
for e1, e2 in zip(lines, lines[1:]):
    diffs[e2 - e1] += 1

print("Part 1:", diffs[1] * diffs[3])

# Part 2
f = lambda i, j: 1 if j > i and lines[j] - lines[i] <= 3 else 0
m = np.fromfunction(np.vectorize(f), (n_lines, n_lines), dtype=int)
aux = np.identity(n_lines)

sol_part_2 = 0
for _ in range(n_lines):
    aux = aux @ m
    sol_part_2 += aux[0, n_lines - 1]

print("Part 2:", int(sol_part_2))
