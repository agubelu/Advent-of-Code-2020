from etc.utils import file_to_lines
from etc.d23 import CrabCupGame

ls = [int(x) for x in file_to_lines("input/day23.txt")[0]]

game = CrabCupGame(ls)
game.play(100)
print("Part 1:", str(game).replace(" ", "")[1:])

big_ls = ls + list(range(10, 1_000_001))
big_game = CrabCupGame(big_ls)
adj = big_game.play(10_000_000, progress_bar=True)

v1 = adj[1]
v2 = adj[v1]
print("Part 2:", v1 * v2)
