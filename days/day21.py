from etc.utils import file_to_lines
from collections import namedtuple
from itertools import chain

Recipe = namedtuple("Recipe", "ingredients, allergens")
lines = file_to_lines("input/day21.txt")

def line_to_recipe(line):
    spl = line.split(" (")
    ingrs = spl[0].split(" ")
    allergs = spl[1] if len(spl) == 2 else()
    
    if allergs:
        allergs = allergs[9:-1].split(", ")

    return Recipe(ingrs, allergs)

recipes = list(map(line_to_recipe, lines))
all_allergens = set(chain.from_iterable(r.allergens for r in recipes))
all_ingredients = set(chain.from_iterable(r.ingredients for r in recipes))

identified_allergens = {a: None for a in all_allergens}
recipes_with_allergen = {a: [r for r in recipes if a in r.allergens] for a in all_allergens}

# Loop until we have identified all allergens
while any(x is None for x in identified_allergens.values()):
    for allergen in (a for a, v in identified_allergens.items() if v is None):
        recipes_alg = recipes_with_allergen[allergen]

        # Get all ingredients shared by recipes that have the relevant allergen, and remove the known ones
        ingrs = set(recipes_alg[0].ingredients)
        ingrs = ingrs.intersection(*(r.ingredients for r in recipes_alg[1:]))
        ingrs -= set(identified_allergens.values())

        if len(ingrs) == 1:
            identified_allergens[allergen] = ingrs.pop()

safe_ingredients = all_ingredients - set(identified_allergens.values())
sol_part_1 = sum(sum(int(ingr in r.ingredients) for ingr in safe_ingredients) for r in recipes)
print("Part 1:", sol_part_1)

ls = list(identified_allergens.items())
ls.sort(key=lambda x: x[0])
sol_part_2 = ",".join(x[1] for x in ls)
print("Part 2:", sol_part_2)
