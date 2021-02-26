use std::hash::Hash;

pub trait Asset: Eq + Hash + Clone {}
