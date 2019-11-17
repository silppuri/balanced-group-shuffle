#[derive(Debug)]
pub struct Shuffle {
    num_groups: usize,
    num_items: usize
}

impl Shuffle {
    pub fn new(groups: &Vec<Vec<usize>>) -> Shuffle {
        let num_items = groups.iter().fold(0, |acc, group| acc + group.len());
        let res: Vec<Vec<usize>> = groups.iter().enumerate().map(|group| {
            let mut result_arr = vec![0; num_items];
            let spread: usize = num_items / group.1.len();

            // /*
            // Create spread arrays with offset of the position of the 
            // group in the initial argument. Also use modulus of the total
            // amount of the items to make it a ring.
            // [1, 0, 2, 0, 0]
            // [0, 1, 0, 2, 0]
            // [0, 0, 1, 0, 2]
            // [2, 0, 0, 1, 0]
            // */

            group.1.iter().enumerate().for_each(|index_item_tuple| {
                result_arr[(group.0 + index_item_tuple.0 * spread) % num_items] = *index_item_tuple.1;
            });
            result_arr
        }).collect();

        // Build the result array without the empty items
        let result = (0..num_items).fold(Vec::with_capacity(num_items), |mut acc, index| {
            res.iter().for_each(|item| {
                if item[index] != 0 {
                    println!("{}", item[index]);
                    acc.push(item[index]);
                }
            });
            acc
        });
        println!("{:?}", result);
        Shuffle { num_groups: groups.len(), num_items: num_items }
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
    assert!(Shuffle::new(&test_data()).num_groups == 4);
    assert!(Shuffle::new(&test_data()).num_items == 12);
}
