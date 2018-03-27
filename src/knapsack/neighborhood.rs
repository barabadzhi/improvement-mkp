use std::sync::RwLock;

use knapsack::item::Item;
use knapsack::rayon::prelude::*;
use knapsack::statistics::Statistics;

#[derive(Debug)]
pub struct Neighborhood<'a> {
    pub result: &'a Statistics,
    pub base_items: Vec<&'a Item>,
    pub neighbors: Vec<(usize, &'a Item)>,
}

impl<'a> Neighborhood<'a> {
    pub fn new(items: &'a [Item], result: &'a Statistics) -> Neighborhood<'a> {
        let (base_items, not_picked_items): (Vec<&Item>, Vec<&Item>) = items
            .into_iter()
            .partition(|&item| result.picked_items.contains(&item.id));

        debug_assert_eq!(
            not_picked_items.len(),
            items.len() - result.picked_items.len()
        );

        let mut neighbors = Vec::with_capacity(result.picked_items.len() * not_picked_items.len());

        for item in not_picked_items {
            for index in 0..result.picked_items.len() {
                let mut neighbor = (index, item);
                neighbors.push(neighbor);
            }
        }

        Neighborhood {
            result,
            base_items,
            neighbors,
        }
    }

    pub fn best_neighbor(&self, capacity: &[u32]) -> Statistics {
        let result = RwLock::new(Statistics::new());

        self.neighbors.par_iter().for_each(|neighbor| {
            let mut capacity_left = capacity.to_vec();
            let mut internal_result = Statistics::new();
            let mut items = self.base_items.clone();

            items[neighbor.0] = neighbor.1;

            for item in items {
                let mut item_can_be_used = false;

                for (constraint_index, constraint) in capacity_left.iter().enumerate() {
                    if item.weights[constraint_index] > *constraint {
                        item_can_be_used = false;
                        break;
                    } else {
                        item_can_be_used = true;
                    }
                }

                if item_can_be_used {
                    for (constraint_index, constraint) in capacity_left.iter_mut().enumerate() {
                        *constraint -= item.weights[constraint_index];
                    }

                    internal_result.picked_items.push(item.id);
                    internal_result.total_profit += item.profit;
                }
            }

            if internal_result.total_profit > result.read().unwrap().total_profit {
                for (left, total) in capacity_left.iter().zip(capacity.iter()) {
                    internal_result.utilization.push(format!(
                        "{:.2}%",
                        ((f64::from(*total - *left) / f64::from(*total)) * 100_f64)
                    ))
                }

                *result.write().unwrap() = internal_result.clone();
            }
        });

        let result = result.into_inner().unwrap();

        if result.total_profit > self.result.total_profit {
            result
        } else {
            self.result.clone()
        }
    }
}
