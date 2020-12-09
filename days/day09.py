from etc.utils import file_to_lines
from itertools import combinations

from timeit import timeit

t1 = timeit()
BUFFER_SIZE = 25
numbers = file_to_lines("input/day09.txt", int)
n_numbers = len(numbers)

# Part 1
def is_valid_pos(pos):
    number = numbers[pos]
    return any(x + y == number for x, y in combinations(numbers[pos - BUFFER_SIZE:pos], 2))

target_number = next(numbers[i] for i in range(BUFFER_SIZE, n_numbers) if not is_valid_pos(i))
print("Part 1:", target_number)

# Part 2
for i in range(n_numbers):
    for j in range(i + 2, n_numbers):
        rnge = numbers[i:j]
        s = sum(rnge)
        if s >= target_number:
            break
    if s == target_number: break

print("Part 2:", min(rnge) + max(rnge))
