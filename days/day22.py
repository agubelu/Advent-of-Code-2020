from etc.utils import file_to_lines
from etc.d22 import Deck, Combat

lines = file_to_lines("input/day22.txt", spl="\n\n")

deck_1 = Deck.from_lines(lines[0].split("\n")[1:])
deck_2 = Deck.from_lines(lines[1].split("\n")[1:])

game1 = Combat(deck_1.copy(), deck_2.copy(), recursive=False)
print("Part 1:", game1.play_game()[1])

game2 = Combat(deck_1.copy(), deck_2.copy(), recursive=True)
print("Part 2:", game2.play_game()[1])
