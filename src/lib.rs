use std::hash::{Hash};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Shuffle {
    num_groups: usize,
    num_items: usize,
    result: Vec<String>
}

impl Shuffle {
    fn build_positioned_items(groups: Vec<Vec<String>>, num_items: usize) -> Vec<HashMap<usize, String>> {
        groups.iter().enumerate().map(|(i, group)| {
            let spread = num_items / group.len();
            let mut positions = HashMap::new();
            for (j, item) in group.iter().enumerate() {
                positions.insert((i + j * spread) % num_items, item.clone());
            }
            positions.clone()
        }).collect()
    }

    pub fn new(groups: Vec<Vec<String>>) -> Shuffle {
        let num_items = groups.iter().fold(0, |acc, group| acc + group.len());
        let num_groups = groups.len();
        let positioned_item_groups = Shuffle::build_positioned_items(groups, num_items);
        let mut result = Vec::new();
        for i in 0..num_items {
            println!("{}", i);
            for group in positioned_item_groups.iter() {
                println!("{:?}", group);
                if let Some(item) = group.get(&i) {
                    result.push(item.clone());
                }
            }
        }
        println!("{:?}", result);
        Shuffle { num_groups: num_groups, num_items: num_items, result: result }
    }
}

pub struct GroupShuffle<K: Hash + Eq, T> {
    groups: HashMap<K, Vec<T>>,
    num_items: usize
}

impl<K: Hash + Eq, T> GroupShuffle<K, T> {
    pub fn new() -> GroupShuffle<K, T> {
        GroupShuffle { groups: HashMap::new(), num_items: 0 }
    }

    pub fn insert(&mut self, key: K, value: T) -> Option<Vec<T>> {   
        self.num_items += 1;
        println!("{}", self.num_items);
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

    pub fn len(&self) -> usize {
        self.groups.len()
    }

    pub fn num_items(&self) -> usize {
        self.num_items
    }
}

#[cfg(test)]
#[test]
fn test_adder() {
    let mut shuffle = GroupShuffle::new();
    shuffle.insert("a", "Sa");
    shuffle.insert("a", "R");
    shuffle.insert("b", "D");
    shuffle.insert("b", "Ja");
    shuffle.insert("b", "A");
    shuffle.insert("c", "Juh");
    shuffle.insert("c", "Jus");
    shuffle.insert("c", "Ar");
    shuffle.insert("c", "P");
    shuffle.insert("d", "N");
    shuffle.insert("d", "T");
    shuffle.insert("d", "Se");
    println!("Length {}", shuffle.len());
    println!("Num items {}", shuffle.num_items());
    assert!(shuffle.len() == 4);
    assert!(shuffle.num_items() == 12);
}