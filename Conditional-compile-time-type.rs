use either::{Left, Right};
use std::{
    any::Any,
    collections::{hash_set::IntoIter, HashSet},
    iter::once,
};
fn main() {
    let mut hashset = HashSet::new();
    for i in 0..10 {
        hashset.insert(i);
    }
    println!("{:?}", once_or_set(hashset.clone()).collect::<Vec<_>>());
    println!("{:?}", set_or_once(hashset).downcast::<IntoIter<i32>>())
}
fn set_or_once(set: HashSet<i32>) -> Box<dyn Any> {
    if 1 == 1 {
        Box::new(set.into_iter())
    } else {
        Box::new(once(1))
    }
}
fn once_or_set(set: HashSet<i32>) -> impl Iterator<Item = i32> {
    if 1 == 1 {
        Left(set.into_iter())
    } else {
        Right(once(1))
    }
}
