import pprint
import functools
import itertools
from scipy.special import comb

def get_assignments(all_ingredients, allergens):
    num_allergens = len(allergens)
    allergens = list(allergens)
    all_ingredients = list(all_ingredients)

    for ingredients in itertools.permutations(all_ingredients, r=num_allergens):
        yield { a : i for (a, i) in zip(allergens, ingredients) }

def part2(data, ingredients_to_consider, assigned_allergens=None):
    if assigned_allergens is None:
        assigned_allergens = {}


    if len(data) == 0:
        return assigned_allergens

    (ingredients, allergens) = data[0]
    ingredients = ingredients_to_consider & ingredients

    for a in allergens:
        if a in assigned_allergens and assigned_allergens[a] not in ingredients:
            return

    for new_assignments in get_assignments(
        ingredients - set(assigned_allergens.values()),
        allergens - set(assigned_allergens.keys()),
    ):
        new_assigned_allergens = assigned_allergens.copy()
        new_assigned_allergens.update(new_assignments)
        res = part2(data[1:], ingredients_to_consider, new_assigned_allergens)
        if res is not None:
            return res
    
    return None

if __name__ == "__main__":

    data = []
    all_ingredients = set()
    with open("inputs.txt", "r") as f:
        for l in f:
            ingredients, allergens = l.split("(")
            allergens = allergens.strip()
            assert allergens.startswith("contains ")
            assert allergens.endswith(")")
            allergens = allergens.replace("contains ", "").replace(")", "")

            ingredients = set(ingredients.split())
            all_ingredients.update(ingredients)
            allergens = set(allergens.split(", "))
            data.append((ingredients, allergens))

    a_to_i = {}
    for ingredients, allergens in data:
        for allergen in allergens:
            if allergen not in a_to_i:
                a_to_i[allergen] = ingredients
            else:
                a_to_i[allergen] = ingredients & a_to_i[allergen]
    
    assigned = functools.reduce(lambda x, acc: x | acc, a_to_i.values())
    not_assigned = all_ingredients - assigned
    
    counter = 0
    for ingredients, _ in data:
        counter += len(ingredients & not_assigned)
    print(counter)

    assigned_allergies = list(part2(data, assigned).items())
    assigned_allergies.sort(key=lambda x: x[0])
    print(",".join([i for (a, i) in assigned_allergies]))

