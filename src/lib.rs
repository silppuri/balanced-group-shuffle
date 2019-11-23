use std::hash::{Hash};
use std::collections::HashMap;

pub struct GroupShuffle<K: Hash + Eq, T: Clone> {
    groups: HashMap<K, Vec<T>>,
    num_values: usize
}

impl<K: Hash + Eq, T: Clone> GroupShuffle<K, T> {
    pub fn new() -> GroupShuffle<K, T> {
        GroupShuffle { groups: HashMap::new(), num_values: 0 }
    }

    pub fn len(&self) -> usize {
        self.groups.len()
    }

    pub fn values(&self) -> Vec<T> {
        self.groups.iter().flat_map(|(_k, v)| v).cloned().collect()
    }

    pub fn values_count(&self) -> usize {
        self.num_values
    }

    pub fn insert(&mut self, key: K, value: T) -> Option<Vec<T>> {   
        self.num_values += 1;
        match self.groups.get_mut(&key) {
            Some(existing_items) => {
                let mut new_items = vec![value];
                new_items.append(existing_items);
                self.groups.insert(key, new_items)
            }
            None => {
                self.groups.insert(key, vec![value])
            }
        }
    }

    pub fn shuffle(&self) -> Vec<T> {
        let positioned_item_groups = self.build_positioned_items();
        let mut result = Vec::new();
        for i in 0..self.num_values {
            for group in positioned_item_groups.iter() {
                if let Some(value) = group.get(&i) {
                    result.push(value.clone());
                }
            }
        }
        result
    }

    fn build_positioned_items(&self) -> Vec<HashMap<usize, T>> {
        self.groups.iter().enumerate().map(|(i, (_key, values))| {
            let spread = self.num_values / values.len();
            let mut positions = HashMap::new();
            for (j, item) in values.iter().enumerate() {
                positions.insert(self.position(i, j, spread), item.clone());
            }
            positions.clone()
        }).collect()
    }

    fn position(&self, i: usize, j: usize, spread: usize) -> usize {
        (i + j * spread) % self.num_values
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