from etc.utils import file_to_lines
import re

password_strings = file_to_lines("input/day02.txt")
REGEX_PWD = re.compile(r"""(\d+)-(\d+) ([a-z]): ([a-z]+)""")

# Part 1
def is_valid_pwd_1(pwd):
    m = REGEX_PWD.match(pwd)
    lower, upper, char, pwd_str = m.groups()
    return pwd_str.count(char) in range(int(lower), int(upper) + 1)

valid_pwds = list(filter(is_valid_pwd_1, password_strings))
print("Part 1:", len(valid_pwds))

# Part 2
def is_valid_pwd_2(pwd):
    m = REGEX_PWD.match(pwd)
    pos1, pos2, char, pwd_str = m.groups()
    return (pwd_str[int(pos1) - 1] == char) != (pwd_str[int(pos2) - 1] == char)  # XOR

valid_pwds = list(filter(is_valid_pwd_2, password_strings))
print("Part 2:", len(valid_pwds))