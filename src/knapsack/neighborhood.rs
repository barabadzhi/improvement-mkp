use knapsack::item::Item;
use knapsack::statistics::Statistics;

#[derive(Debug)]
pub struct Neighborhood<'a> {
    pub original_solution: &'a Statistics,
    pub neighbors: Vec<(usize, &'a Item)>,
}

impl<'a> Neighborhood<'a> {
    pub fn new(items: &'a [Item], original_solution: &'a Statistics) -> Neighborhood<'a> {
        let not_picked_items: Vec<&Item> = items.into_iter().filter(|&item| !original_solution.picked_items.contains(&item.id)).collect();

        debug_assert_eq!(not_picked_items.len(), items.len() - original_solution.picked_items.len());

        let mut neighbors = Vec::with_capacity(original_solution.picked_items.len() * not_picked_items.len());

        for item in not_picked_items {
            for index in 0..original_solution.picked_items.len() {
                let mut neighbor = (index, item);
                neighbors.push(neighbor);
            }
        }

        Neighborhood {
            original_solution,
            neighbors,
        }
    }
}
