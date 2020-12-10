from etc.utils import file_to_lines
import networkx as nx
import numpy as np

joltages = set(file_to_lines("input/day10.txt", func=int))
initial = 0
target = max(joltages) + 3
nodes = {initial}.union(joltages)

# Build the graph
graph = nx.DiGraph()
for node in nodes:
    for k in range(node + 1, node + 4):
        if k in nodes:
            graph.add_edge(node, k)
graph.add_edge(target - 3, target)

# Part 1
diffs = {}
cur_node = initial
while cur_node != target:
    neighbors = list(graph.neighbors(cur_node))
    next_node = min(neighbors)
    diff = next_node - cur_node
    diffs[diff] = diffs.get(diff, 0) + 1
    cur_node = next_node

print("Part 1:", diffs[1] * diffs[3])

# Part 2
adj_matrix = nx.to_numpy_matrix(graph)
start_pow = len(nx.shortest_path(graph, initial, target)) - 1
last_elem = adj_matrix.shape[0] - 1

aux_mat = np.linalg.matrix_power(adj_matrix, start_pow)
n_paths = aux_mat[0, last_elem]

for i in range(start_pow, last_elem):
    aux_mat = aux_mat @ adj_matrix
    n_paths += aux_mat[0, last_elem]

print("Part 2:", int(n_paths))
