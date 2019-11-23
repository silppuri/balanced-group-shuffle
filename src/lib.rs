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
                positions.insert((i + j * spread) % self.num_values, item.clone());
            }
            positions.clone()
        }).collect()
    }

}

#[cfg(test)]
#[test]
fn test_adder() {
    let mut shuffle = GroupShuffle::new();
    shuffle.insert("a", "a");
    shuffle.insert("a", "b");
    shuffle.insert("b", "c");
    shuffle.insert("b", "d");
    shuffle.insert("b", "e");
    shuffle.insert("c", "f");
    shuffle.insert("c", "g");
    shuffle.insert("c", "h");
    shuffle.insert("c", "i");
    shuffle.insert("d", "j");
    shuffle.insert("d", "k");
    shuffle.insert("d", "l");
    assert!(shuffle.len() == 4);
    assert!(shuffle.values().len() == 12);
    assert!(shuffle.shuffle().len() == 12);
}