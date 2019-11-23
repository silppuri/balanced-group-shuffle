use std::hash::{Hash};
use std::collections::HashMap;

pub struct GroupShuffle<K: Hash + Eq, T> {
    groups: HashMap<K, Vec<T>>,
    num_items: usize
}

impl<K: Hash + Eq, T> GroupShuffle<K, T> {
    pub fn new() -> GroupShuffle<K, T> {
        GroupShuffle { groups: HashMap::new(), num_items: 0 }
    }

    pub fn len(&self) -> usize {
        self.groups.len()
    }

    pub fn num_items(&self) -> usize {
        self.num_items
    }

    pub fn insert(&mut self, key: K, value: T) -> Option<Vec<T>> {   
        self.num_items += 1;
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

    pub fn shuffle(&self) -> Vec<&T> {
        let positioned_item_groups = self.build_positioned_items();
        let mut result = Vec::new();
        for i in 0..self.num_items {
            for group in positioned_item_groups.iter() {
                if let Some(item) = group.get(&i) {
                    result.push(item.clone());
                }
            }
        }
        result
    }

    fn build_positioned_items(&self) -> Vec<HashMap<usize, &T>> {
        self.groups.iter().enumerate().map(|(i, (_key, items))| {
            let spread = self.num_items / items.len();
            let mut positions = HashMap::new();
            for (j, item) in items.iter().enumerate() {
                positions.insert((i + j * spread) % self.num_items, item.clone());
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
    println!("Length {}", shuffle.len());
    println!("Num items {}", shuffle.num_items());
    assert!(shuffle.len() == 4);
    assert!(shuffle.num_items() == 12);
    assert!(shuffle.shuffle().len() == 12);
    println!("Num items {:?}", shuffle.shuffle());
}