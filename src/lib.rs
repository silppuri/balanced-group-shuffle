#[derive(Debug)]
pub struct Shuffle {
    num_groups: usize,
    total_items: usize
}

impl Shuffle {
    pub fn new(groups: Vec<Vec<usize>>) -> Shuffle {
        Shuffle { num_groups: groups.len(), total_items: 0 }
    }
}

#[cfg(test)]

fn test_data() -> Vec<Vec<usize>> {
    let a = vec![1,2];
    let b = vec![3,4,5];
    let c = vec![6,7,8,9];
    let d = vec![10,11,12];

    vec![a,b,c,d]
}

#[test]
fn test_adder() {
    assert!(Shuffle::new(test_data()).num_groups == 4);
    assert!(Shuffle::new(test_data()).total_items == 12);
}
