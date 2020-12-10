from etc.utils import file_to_lines
import networkx as nx
import numpy as np

lines = [0] + file_to_lines("input/day10.txt", func=int)
n_lines = len(lines)
lines.sort()
target = lines[-1]

# Part 1
diffs = {1: 0, 2: 0, 3: 1}
for i in range(1, n_lines):
    diff = lines[i] - lines[i - 1]
    diffs[diff] += 1

print("Part 1:", diffs[1] * diffs[3])

# Part 2
graph = nx.DiGraph()
for i in range(n_lines):
    node = lines[i]
    for other in lines[i + 1:i + 4]:
        if other - node > 3: break
        graph.add_edge(node, other)

adj_matrix = nx.to_numpy_matrix(graph)
start_pow = len(nx.shortest_path(graph, 0, target)) - 1
last_elem = adj_matrix.shape[0] - 1

aux_mat = np.linalg.matrix_power(adj_matrix, start_pow)
n_paths = aux_mat[0, last_elem]

for i in range(start_pow, last_elem):
    aux_mat = aux_mat @ adj_matrix
    n_paths += aux_mat[0, last_elem]

print("Part 2:", int(n_paths))
