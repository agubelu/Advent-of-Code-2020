from etc.utils import file_to_lines
from etc.handheld_console import HandheldConsole
import networkx as nx

# Parsing
lines = file_to_lines("input/day08.txt")
instructions = list(map(lambda l: (l[:3], int(l[4:])), lines))
last_instr = len(instructions) - 1

# Building the instruction flow graph
graph = nx.Graph()
for i, (instr, val) in enumerate(instructions):
    target = i + 1 if instr != "jmp" else i + val
    graph.add_edge(i, target)

# Part 1
cons = HandheldConsole(instructions)
print("Part 1:", cons.run())

# Part 2
# This works under the assumption that the first and last instructions are in
# different connected components in the instruction flow graph, which
# is true for the example and my input.
# If it's not true for yours then ¯\_(ツ)_/¯
comp_start = nx.node_connected_component(graph, 0)
comp_end = nx.node_connected_component(graph, last_instr)

for i in comp_start:
    instr, val = instructions[i]
    if instr == "acc": continue
    alt_instr = "jmp" if instr == "nop" else "nop"
    alt_target = i + 1 if alt_instr == "nop" else i + val
    
    if alt_target in comp_end:
        # This is the instruction that we should change
        instructions[i] = (alt_instr, val)

print("Part 2:", cons.run(yolo=True))
