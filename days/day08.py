from etc.utils import file_to_lines
import networkx as nx

# Parsing
lines = file_to_lines("input/day08.txt")
instructions = list(map(lambda l: (l[:3], int(l[4:])), lines))
last_instr = len(instructions) - 1

# Building the instruction flow graph
graph = nx.DiGraph()
for i, (instr, val) in enumerate(instructions):
    target = i + 1 if instr != "jmp" else i + val
    graph.add_edge(i, target, object=(instr, val))

# Navigation function for part 1
def nav_part_1(graph, initial=0):
    cur_node = initial
    cand_edges = []
    visited_nodes = set()
    acc = 0

    while cur_node not in visited_nodes:
        visited_nodes.add(cur_node)
        _, target, edge_data = list(graph.out_edges(cur_node, data="object"))[0] # Each node has exactly 1 outgoing edge

        if edge_data[0] == "acc":
            acc += edge_data[1]
        else:
            cand_edges.append((cur_node, target, edge_data))

        cur_node = target
        
    return acc, cand_edges

# Navigation function for part 2
def nav_part_2(graph, initial=0, objective=last_instr):
    cur_node = initial
    acc = 0

    while True:
        _, target, edge_data = list(graph.out_edges(cur_node, data="object", default=("nop", 0)))[0] # Each node has exactly 1 outgoing edge

        if edge_data[0] == "acc":
            acc += edge_data[1]

        if cur_node == objective: break
        cur_node = target
        
    return acc

# Part 1
sol_part_1, cand_edges = nav_part_1(graph)
print("Part 1:", sol_part_1)

# Part 2
for source, target, (instr, val) in cand_edges:
    # Remove the edge and add the opposite one
    new_target = source + 1 if instr == "jmp" else source + val
    if new_target < 0: continue

    graph.remove_edge(source, target)
    graph.add_edge(source, new_target)

    # Check for connectivity
    if nx.has_path(graph, 0, last_instr):
        break

    # Undo the change
    graph.remove_edge(source, new_target)
    graph.add_edge(source, target)


sol_part_2 = nav_part_2(graph)
print("Part 2:", sol_part_2)
