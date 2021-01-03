use std::fs::read_to_string;
use std::time::Instant;

use rustc_hash::{FxHashSet, FxHashMap};

///////////////////////////////////////////////////////////////////////////////

struct Recipe<'a> {
    ingredients: FxHashSet<&'a str>,
    allergens: FxHashSet<&'a str>,
}

type AllergenSetMap<'a> = FxHashMap<&'a str, FxHashSet<&'a str>>;
type AllergenMap<'a> = FxHashMap<&'a str, &'a str>;

///////////////////////////////////////////////////////////////////////////////

pub fn run() {
    let time = Instant::now();

    let input = read_to_string("../input/day21.txt").expect("Error reading file");
    let recipes = read_data(&input);
    
    let common_ingrs = get_common_ingredients_allergens(&recipes);
    let identified_allergens = identify_allergens(&common_ingrs);

    let bad_ingredients: FxHashSet<&str> = identified_allergens.values().copied().collect();
    let sol_part_1: usize = recipes.iter()
                                   .map(|rec| rec.ingredients.difference(&bad_ingredients).count())
                                   .sum();

    let mut ingr_list = identified_allergens.iter().collect::<Vec<_>>();
    ingr_list.sort_by_key(|p| p.0);

    let sol_part_2 = ingr_list.iter()
                              .map(|p| *p.1)
                              .collect::<Vec<_>>()
                              .join(",");

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}

///////////////////////////////////////////////////////////////////////////////

fn identify_allergens<'a>(common_ingrs: &'a AllergenSetMap) -> AllergenMap<'a> {
    let mut res: AllergenMap<'a> = FxHashMap::default();
    let n_allergens = common_ingrs.len();
    
    while res.len() != n_allergens {
        let remaining_allergs = common_ingrs.keys().filter(|&a| !res.contains_key(a)).collect::<Vec<_>>();
        
        for &allerg in remaining_allergs {
            let identified = res.values().cloned().collect();
            let possible_ingrs = common_ingrs[allerg].difference(&identified).collect::<Vec<_>>();
            if possible_ingrs.len() == 1 {
                res.insert(allerg, *possible_ingrs[0]);
            }
        }
    }

    return res;
}

fn get_common_ingredients_allergens<'a>(recipes: &'a[Recipe]) -> AllergenSetMap<'a> {
    let mut map: AllergenSetMap<'a> = FxHashMap::default();

    for recipe in recipes {
        for allergen in &recipe.allergens {
            let common = match map.get(*allergen) {
                None => recipe.ingredients.clone(),
                Some(ingrs) => ingrs.intersection(&recipe.ingredients).copied().collect(),
            };

            map.insert(*allergen, common);
        }
    }

    return map;
}

fn read_data(data: &str) -> Vec<Recipe> {
    data.lines().map(|line| {
        let spl = line.split("(contains ").collect::<Vec<_>>();
        let ingredients = spl[0].trim().split(' ').collect();

        let allergens = if spl.len() == 1 {
            FxHashSet::default()
        } else {
            let len = spl[1].len();
            spl[1][..len-1].trim().split(", ").collect()
        };

        Recipe { ingredients, allergens }
    }).collect()
}
