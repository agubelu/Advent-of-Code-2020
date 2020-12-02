from etc.utils import file_to_lines
import re

password_strings = file_to_lines("input/day02.txt")
REGEX_PWD = re.compile(r"""(\d+)-(\d+) ([a-z]): ([a-z]+)""")

# Parts 1 & 2
correct_1 = correct_2 = 0
for pwd in password_strings:
    m = REGEX_PWD.match(pwd)
    lower, upper, char, pwd_str = m.groups()
    lower = int(lower)
    upper = int(upper)

    if pwd_str.count(char) in range(lower, upper + 1):
        correct_1 += 1

    if (pwd_str[lower - 1] == char) != (pwd_str[upper - 1] == char):  # XOR
        correct_2 += 1

print("Part 1:", correct_1)
print("Part 2:", correct_2)