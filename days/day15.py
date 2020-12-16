from etc.utils import file_to_lines

ITERS = 30_000_000

initial = file_to_lines("input/day15.txt", func=int, spl=",")
mem = {n: i + 1 for i, n in enumerate(initial)}
last_turn = len(initial) + 2
cur_num = 0

for turn in range(last_turn, ITERS + 1):
    last_seen = mem.get(cur_num, None)
    mem[cur_num] = turn - 1
    cur_num = turn - last_seen - 1 if last_seen else 0

    if turn == 2020:
        print("Part 1:", cur_num)

print("Part 2:", cur_num)
