use std::{collections::HashSet, hash::Hash};

pub fn vector_difference<T: Clone + Eq + Hash>(v1: &Vec<T>, v2: &Vec<T>) -> Vec<T> {
    let s1: HashSet<T> = v1.iter().cloned().collect();
    let s2: HashSet<T> = v2.iter().cloned().collect();
    (&s1 - &s2).iter().cloned().collect()
}
