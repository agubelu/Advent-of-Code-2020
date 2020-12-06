from etc.utils import file_to_lines

lines = file_to_lines("input/day06.txt", spl="\n\n")
sets = [[set(line) for line in group.split("\n")] for group in lines]

# Part 1
unique_answers = [set().union(*group) for group in sets]
print("Part 1:", sum(len(x) for x in unique_answers))

# Part 2
common_answers = [group[0].intersection(*group[1:]) for group in sets]
print("Part 2:", sum(len(x) for x in common_answers))
