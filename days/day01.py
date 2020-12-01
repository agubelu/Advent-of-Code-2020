from etc.utils import file_to_lines
import sys

numbers = file_to_lines("input/day01.txt", int)
n_numbers = len(numbers)

# Part 1
found = False
for i in range(n_numbers):
    for j in range(i + 1, n_numbers):
        if numbers[i] + numbers[j] == 2020:
            print("Part 1:", numbers[i] * numbers[j])
            found = True
            break

    if found: break

# Part 2
for i in range(n_numbers):
    for j in range(i + 1, n_numbers):
        if numbers[i] + numbers[j] >= 2020: continue

        for k in range(j + 1, n_numbers):
            if numbers[i] + numbers[j] + numbers[k] == 2020:
                print("Part 2:", numbers[i] * numbers[j] * numbers[k])
                sys.exit()
