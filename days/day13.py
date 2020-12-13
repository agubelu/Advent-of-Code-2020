# This code uses stuff that requires Python >= 3.8 !!!
from etc.utils import file_to_lines
from math import prod

lines = file_to_lines("input/day13.txt")
tstamp = int(lines[0])
spl = lines[1].split(",")
buses = [int(x) for x in spl if x != "x"]
inds = [i for i in range(len(spl)) if spl[i] != "x"]

wait_times = [x - (tstamp % x) for x in buses]
shortest_wait = min(wait_times)
line = buses[wait_times.index(shortest_wait)]
print("Part 1:", shortest_wait * line)

# Part 2 is basically a congruency system, where the modulos are the buses'
# periods and the remainders are the periods minus the offset in which they
# appear in the list (modulo the period if it's negative).

# It seems that all periods for the buses are prime numbers, so we can apply
# the chinese remainder theorem to solve the congruency system and get
# the timestamp value that meets the criteria

def crt(modulos, remainders):
    N = prod(modulos)
    Ni = [N // m for m in modulos]
    Xi = [pow(ni, -1, m) for ni, m in zip(Ni, modulos)]
    s = sum(ri * ni * xi for ri, ni, xi in zip(remainders, Ni, Xi))
    return s % N

rems = [(buses[i] - inds[i]) % buses[i] for i in range(len(buses))]
print("Part 2:", crt(buses, rems))