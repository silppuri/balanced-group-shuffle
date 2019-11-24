use rand::prelude::*;
use rand::{thread_rng, Rng};
use std::hash::Hash;
use std::collections::BTreeMap;

pub struct BalancedGroupShuffle<K: Hash + Eq + Ord, T: Clone> {
    groups: BTreeMap<K, Vec<T>>,
    num_values: usize
}

impl<K: Hash + Eq + Ord, T: Clone> BalancedGroupShuffle<K, T> {
    pub fn new() -> BalancedGroupShuffle<K, T> {
        BalancedGroupShuffle { groups: BTreeMap::new(), num_values: 0 }
    }

    pub fn len(&self) -> usize {
        self.groups.len()
    }

    pub fn values(&self) -> Vec<T> {
        self.groups.iter().flat_map(|(_k, v)| v).cloned().collect()
    }

    pub fn insert(&mut self, key: K, value: T) -> Option<Vec<T>> {   
        self.num_values += 1;
        BalancedGroupShuffle::value_into(&mut self.groups, key, value)
    }

    pub fn shuffle(&self) -> Vec<&T> {
        self.disperse_values().iter().flat_map(|(_position, values)| values).cloned().collect()
    }

    fn disperse_values(&self) -> BTreeMap<usize, Vec<&T>> {
        let mut map = BTreeMap::new();
        for (_i, (_key, values)) in self.groups.iter().enumerate() {
            let spread = self.num_values / values.len();
            for (j, value) in values.iter().enumerate() {
                let position = self.position(j, spread);
                BalancedGroupShuffle::value_into(&mut map, position, value);
            }
        }
        map
    }

    fn position(&self, j: usize, spread: usize) -> usize {
        let mut rng = thread_rng();
        let randomness = 0.05;
        if j == 0 {
            rng.gen_range(0, self.len())
        } else {
            let lower = spread as f64 * j as f64 - spread as f64 * randomness;
            let upper = spread as f64 * j as f64 + spread as f64 * randomness;
            rng.gen_range(lower, upper) as usize
        }
    }

    fn value_into(map: &mut BTreeMap<K, Vec<T>>, key: K, value: T) -> Option<Vec<T>> {
        match map.get_mut(&key) {
            Some(values) => {
                let mut rng = thread_rng();
                let mut new_values = vec![];
                new_values.append(values);
                new_values.append(&mut vec![value]);
                new_values.shuffle(&mut rng);
                map.insert(key, new_values)
            }
            None => {
                map.insert(key, vec![value])
            }
        }
    }
}

#[cfg(test)]
fn test_data() -> BalancedGroupShuffle<&'static str, &'static str> {
    let mut shuffle = BalancedGroupShuffle::new();
    shuffle.insert("a", "a1");
    shuffle.insert("a", "a2");
    shuffle.insert("b", "b1");
    shuffle.insert("b", "b2");
    shuffle.insert("b", "b3");
    shuffle.insert("c", "c1");
    shuffle.insert("c", "c2");
    shuffle.insert("c", "c3");
    shuffle.insert("c", "c4");
    shuffle.insert("d", "d1");
    shuffle.insert("d", "d2");
    shuffle.insert("d", "d3");
    shuffle
}

#[test]
fn test_insert() {
    let shuffle = test_data();
    assert!(shuffle.len() == 4);
    assert!(shuffle.values().len() == 12);
}

#[test]
fn test_shuffle() {
    let shuffle = test_data();
    assert!(shuffle.shuffle().len() == 12);
}