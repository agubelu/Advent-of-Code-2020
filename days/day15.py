from etc.utils import file_to_lines

ITERS = 30_000_000

initial = file_to_lines("input/day15.txt", func=int, spl=",")
mem = {n: i + 1 for i, n in enumerate(initial)}
turn = len(initial) + 2
cur_num = 0

while turn <= ITERS:
    last_seen = mem.get(cur_num, 0)
    mem[cur_num] = turn - 1
    cur_num = 0 if last_seen == 0 else turn - last_seen - 1

    if turn == 2020:
        print("Part 1:", cur_num)
        
    turn += 1

print("Part 2:", cur_num)
