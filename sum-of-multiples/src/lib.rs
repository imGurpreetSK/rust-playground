use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set = HashSet::new();
    factors.iter().filter(|x| **x != 0).for_each(|factor| {
        (*factor..limit).filter(|x| x % factor == 0).for_each(|x| { set.insert(x); });
    });

    set.iter().sum::<u32>()
}
