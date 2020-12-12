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
f = lambda i, j: j > i and lines[j] - lines[i] <= 3
mat = np.fromfunction(np.vectorize(f), (n_lines, n_lines), dtype=np.int64).astype(np.int64)
mat[n_lines - 1, n_lines - 1] = 1
mat = np.linalg.matrix_power(mat, n_lines)

print("Part 2:", mat[0, n_lines - 1])
