from etc.utils import file_to_lines
import numpy as np

initial = file_to_lines("input/day11.txt")
shape = (len(initial), len(initial[0]))

func_initial = lambda i, j: {".": 0, "L": 1, "#": 2}[initial[i][j]]
initial_mat = np.fromfunction(np.vectorize(func_initial), shape, dtype=int)
mat = initial_mat.copy()

def func_next_1(i, j):
    val = mat[i, j]
    if val == 0: return val
    adj = 0

    for n in range(max(0, i - 1), min(shape[0], i + 2)):
        for m in range(max(0, j - 1), min(shape[1], j + 2)):
            if (n != i or m != j) and mat[n,m] == 2:
                adj += 1

    if val == 1 and adj == 0:
        return 2
    elif val == 2 and adj >= 4:
        return 1
    else:
        return val

f1 = np.vectorize(func_next_1)
next_mat = np.fromfunction(np.vectorize(f1), shape, dtype=int)
while np.any(next_mat != mat):
    mat = next_mat
    next_mat = np.fromfunction(np.vectorize(f1), shape, dtype=int)

occupied = np.count_nonzero(mat == 2)
print("Part 1:", occupied)

mat = initial_mat.copy()
def func_next_2(i, j):
    val = mat[i, j]
    if val == 0: return val
    adj = 0
    diffs = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)]

    for dx, dy in diffs:
        n, m = i+dx, j+dy
        while 0 <= n < shape[0] and 0 <= m < shape[1]:
            s = mat[n, m]
            if s == 1:
                break
            elif s == 2:
                adj += 1
                break
            n += dx
            m += dy
        
    if val == 1 and adj == 0:
        return 2
    elif val == 2 and adj >= 5:
        return 1
    else:
        return val


f2 = np.vectorize(func_next_2)
next_mat = np.fromfunction(np.vectorize(f2), shape, dtype=int)
mat = next_mat
next_mat = np.fromfunction(np.vectorize(f2), shape, dtype=int)

while np.any(next_mat != mat):
    mat = next_mat
    next_mat = np.fromfunction(np.vectorize(f2), shape, dtype=int)

occupied = np.count_nonzero(mat == 2)
print("Part 2:", occupied)