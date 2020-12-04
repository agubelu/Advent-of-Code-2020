from etc.utils import file_to_lines
import re

passports_raw = file_to_lines("input/day04.txt", spl="\n\n")
RE_PASS = re.compile(r"""([a-z]{3}):(.*?)(?:\s|$)""")

def line_to_passport(line):
    return dict(m.groups() for m in RE_PASS.finditer(line))
passports = list(map(line_to_passport, passports_raw))

regexs = {
    "byr": re.compile(r"""^(19[2-9]\d|200[0-2])$"""),
    "iyr": re.compile(r"""^(201\d|2020)$"""),
    "eyr": re.compile(r"""^(202\d|2030)$"""),
    "hgt": re.compile(r"""^(1([5-8]\d|9[0-3])cm|(59|6\d|7[0-6])in)$"""),
    "hcl": re.compile(r"""^#[0-9a-f]{6}$"""),
    "ecl": re.compile(r"""^(amb|blu|brn|gry|grn|hzl|oth)$"""),
    "pid": re.compile(r"""^[0-9]{9}$"""),
}

# Parts 1 & 2
is_valid_passport_1 = lambda pp: all(f in pp for f in regexs)
is_valid_passport_2 = lambda pp: all(rgx.match(pp[k]) for k, rgx in regexs.items())  # Should only be applied to passports that meet the previous criterion

valid_1 = valid_2 = 0

for pp in passports:
    if is_valid_passport_1(pp):
        valid_1 += 1
        if is_valid_passport_2(pp):
            valid_2 += 1

print("Part 1:", valid_1)
print("Part 2:", valid_2)
