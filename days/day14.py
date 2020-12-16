from etc.utils import file_to_lines
from itertools import product
import re

RE_MEM = re.compile(r"""mem\[(\d+)\] = (\d+)""")
ADDR_SIZE = 36

# I worked with integers in Rust so let's work with strings here
# just to have some variety

lines = file_to_lines("input/day14.txt")
num2bin = lambda n: bin(n)[2:].zfill(ADDR_SIZE)
bin2num = lambda b: int(b, 2)

def apply_mask_v1(s, mask):
    return "".join(s[i] if mask[i] == "X" else mask[i] for i in range(len(mask)))

def apply_mask_v2(s, mask):
    masked = "".join(mask[i] if mask[i] != "0" else s[i] for i in range(len(mask)))
    n_vars = masked.count("X")
    masked = masked.replace("X", "{}")
    return [masked.format(*x) for x in product("01", repeat=n_vars)]

mem1 = {}
mem2 = {}
mask = None

for line in lines:
    if line.startswith("mask"):
        mask = line[7:]
        continue

    addr, val = [int(x) for x in RE_MEM.match(line).groups()]
    
    mem1[addr] = bin2num(apply_mask_v1(num2bin(val), mask))
    for alt_addr in apply_mask_v2(num2bin(addr), mask):
        mem2[alt_addr] = val

print("Part 1:", sum(mem1.values()))
print("Part 2:", sum(mem2.values()))
