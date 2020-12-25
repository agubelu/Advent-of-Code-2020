from etc.utils import file_to_lines
from sympy.ntheory import discrete_log

card_pk, door_pk = file_to_lines("input/day25.txt", func=int)
card_loop = discrete_log(20201227, card_pk, 7)
key = pow(door_pk, card_loop, 20201227)
print("Part 1:", key)
