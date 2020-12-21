from etc.utils import file_to_lines
from etc.d18 import Val
import re

repl = lambda line: re.sub(r"(\d+)", r"Val(\1)", line)
lines = file_to_lines("input/day18.txt", func=repl)

def solve(repl_sum):
    exprs = [eval(line.replace("+", repl_sum)) for line in lines]
    return sum(x.eval() for x in exprs)

print("Part 1:", solve("//"))
print("Part 2:", solve("**"))
