from etc.utils import file_to_lines
import re

RE_OUTER = re.compile(r"(.*) bags contain")
RE_INNER = re.compile(r"(\d+) (.*?) bags?")

# Parsing
lines = file_to_lines("input/day07.txt")
data_tree = {}

def init_color(color):
    if color not in data_tree:
        data_tree[color] = {"contained_by": set(), "contains": []}

for line in lines:
    outer_color = RE_OUTER.match(line).group(1)  # Will always match
    init_color(outer_color)

    for m in RE_INNER.finditer(line):
        num, inner_color = int(m.group(1)), m.group(2)
        init_color(inner_color)
        data_tree[outer_color]["contains"].append((num, inner_color))
        data_tree[inner_color]["contained_by"].add(outer_color)

# Part 1
visited_parents = set()

def visit_parents(color):
    parents_to_visit = [x for x in data_tree[color]["contained_by"] if x not in visited_parents]
    for p in parents_to_visit:
        visited_parents.add(p)
        visit_parents(p)

visit_parents("shiny gold")
print("Part 1:", len(visited_parents))

# Part 2
def count_inner_bags(color):
    res = 1
    contains = data_tree[color]["contains"]
    if not contains: return res

    for num, inner_color in contains:
        res += num * count_inner_bags(inner_color)

    return res

num_inner_bags = count_inner_bags("shiny gold") - 1
print("Part 2:", num_inner_bags)
