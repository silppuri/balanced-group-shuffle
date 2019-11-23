use rand::{thread_rng, Rng};
use std::hash::Hash;
use std::collections::BTreeMap;

pub struct GroupShuffle<K: Hash + Eq + Ord, T: Clone> {
    groups: BTreeMap<K, Vec<T>>,
    num_values: usize
}

impl<K: Hash + Eq + Ord, T: Clone> GroupShuffle<K, T> {
    pub fn new() -> GroupShuffle<K, T> {
        GroupShuffle { groups: BTreeMap::new(), num_values: 0 }
    }

    pub fn len(&self) -> usize {
        self.groups.len()
    }

    pub fn values(&self) -> Vec<T> {
        self.groups.iter().flat_map(|(_k, v)| v).cloned().collect()
    }

    pub fn insert(&mut self, key: K, value: T) -> Option<Vec<T>> {   
        self.num_values += 1;
        GroupShuffle::value_into_map_vector(&mut self.groups, key, value)
    }

    pub fn shuffle(&self) -> Vec<&T> {
        let positioned_item_groups = self.disperse_values();
        positioned_item_groups.iter().flat_map(|(_position, values)| values).cloned().collect()
    }

    fn disperse_values(&self) -> BTreeMap<usize, Vec<&T>> {
        let mut map = BTreeMap::new();
        for (_i, (_key, values)) in self.groups.iter().enumerate() {
            let spread = self.num_values / values.len();
            for (j, value) in values.iter().enumerate() {
                let position = self.position(j, spread);
                GroupShuffle::value_into_map_vector(&mut map, position, value);
            }
        }
        map
    }

    fn position(&self, j: usize, spread: usize) -> usize {
        let mut rng = thread_rng();
        if j == 0 {
            rng.gen_range(0, self.len())
        } else {
            let lower = spread as f64 * j as f64 - self.num_values as f64 * 0.05;
            let upper = spread as f64 * j as f64 + self.num_values as f64 * 0.05;
            rng.gen_range(lower, upper) as usize
        }
    }

    fn value_into_map_vector(map: &mut BTreeMap<K, Vec<T>>, key: K, value: T) -> Option<Vec<T>> {
        match map.get_mut(&key) {
            Some(values) => {
                let mut new_values = vec![];
                new_values.append(values);
                new_values.append(&mut vec![value]);
                map.insert(key, new_values)
            }
            None => {
                map.insert(key, vec![value])
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test_adder() {
    let mut shuffle = GroupShuffle::new();
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
    assert!(shuffle.len() == 4);
    assert!(shuffle.values().len() == 12);
    assert!(shuffle.shuffle().len() == 12);
    println!("{:?}", shuffle.shuffle());
}