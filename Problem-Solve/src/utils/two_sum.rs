use std::collections::{HashMap, hash_map::Entry};

pub fn two_sum(nums: Vec<i32>, t: i32) -> Vec<i32> {
    nums.into_iter()
        .enumerate()
        .try_fold(HashMap::new(), |mut map, (i, n)| {
            match map.entry(t - n) {
                Entry::Vacant(_) => {
                    map.insert(n, i);
                    Ok(map)
                },
                Entry::Occupied(j) => Err(vec![i as i32, *j.get() as i32]),
            }
        })
        .err()
        .unwrap()
}