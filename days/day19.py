from etc.utils import file_to_lines
from etc.d19 import CompositePattern, TextPattern
import regex as re

RE_PAT = re.compile(r"(\d+): (.*)")
RE_VAL = re.compile(r'"(.*)"')

data = file_to_lines("input/day19.txt", spl="\n\n")
patterns = list(data[0].split("\n"))
lines = list(data[1].split("\n"))

# Parsing rules
rules = {}
for pat in patterns:
    i, text = RE_PAT.match(pat).groups()
    m = RE_VAL.match(text)

    if m:
        text = m.group(1)
        rules[int(i)] = TextPattern(text)
    else:
        rules[int(i)] = CompositePattern(text)

def count_matches(regex, lines):
    return sum(int(bool(regex.match(line))) for line in lines)

# Part 1
re_0_text = rules[0].to_regex(rules)
re_part_1 = re.compile(re_0_text + "$")
print("Part 1:", count_matches(re_part_1, lines))

# Part 2
r42 = rules[42].to_regex(rules)
r31 = rules[31].to_regex(rules)
re_part_2 = re.compile(f"({r42})+(?<rec>({r42})(?&rec)?({r31}))$")
print("Part 2:", count_matches(re_part_2, lines))
