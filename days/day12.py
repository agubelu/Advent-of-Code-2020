from etc.utils import file_to_lines

MOVES = [(0, -1), (+1, 0), (0, +1), (-1, 0)]
DIRS = {"N": 0, "E": 1, "S": 2, "W": 3}

lines = file_to_lines("input/day12.txt")
line_to_instr = lambda l: (l[0], int(l[1:]))
instructions = list(map(line_to_instr, lines))

cur_heading = 90
cur_pos = (0, 0)

for instr, val in instructions:
    if instr in DIRS:
        move = MOVES[DIRS[instr]]
        cur_pos = cur_pos[0] + move[0] * val, cur_pos[1] + move[1] * val
    elif instr in ("L", "R"):
        if instr == "L":
            val = -val
        cur_heading = (cur_heading + val) % 360
    else:  # F
        move = MOVES[cur_heading // 90]
        cur_pos = cur_pos[0] + move[0] * val, cur_pos[1] + move[1] * val

print("Part 1:", sum(abs(x) for x in cur_pos))

wpt_pos = (10, -1)
cur_pos = (0, 0)

for instr, val in instructions:
    if instr in DIRS:
        move = MOVES[DIRS[instr]]
        wpt_pos = wpt_pos[0] + move[0] * val, wpt_pos[1] + move[1] * val
    elif instr in ("L", "R"):
        mult = (1, -1) if instr == "L" else (-1, 1)
        for _ in range(val // 90):
            wpt_pos = wpt_pos[1] * mult[0], wpt_pos[0] * mult[1]
    else:  # F
        cur_pos = cur_pos[0] + wpt_pos[0] * val, cur_pos[1] + wpt_pos[1] * val

print("Part 2:", sum(abs(x) for x in cur_pos))
