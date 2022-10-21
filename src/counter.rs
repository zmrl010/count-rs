use std::{
    borrow::Borrow,
    collections::hash_map::{IntoIter, Iter},
    hash::Hash,
    iter::Sum,
    ops::{AddAssign, Index, IndexMut},
};

use ahash::AHashMap;
use num_traits::{One, Zero};

/// Struct for counting hash-able objects or primitives
///
/// Uses [`std::collections::HashMap`] underneath,
/// also borrowing some of it's api
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Counter<T, C = usize>
where
    T: Hash + Eq,
{
    map: AHashMap<T, C>,
    zero: C,
}

impl<T, Q, C> Index<&'_ Q> for Counter<T, C>
where
    T: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq,
    C: Zero,
{
    type Output = C;

    fn index(&self, key: &'_ Q) -> &Self::Output {
        self.map.get(key).unwrap_or(&self.zero)
    }
}

impl<T, Q, C> IndexMut<&'_ Q> for Counter<T, C>
where
    T: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq + ToOwned<Owned = T>,
    C: Zero,
{
    fn index_mut(&mut self, key: &'_ Q) -> &mut C {
        self.map.entry(key.to_owned()).or_insert_with(C::zero)
    }
}

impl<T, C> Default for Counter<T, C>
where
    T: Hash + Eq,
    C: Default,
{
    fn default() -> Self {
        Self {
            map: Default::default(),
            zero: Default::default(),
        }
    }
}

impl<T, C> Counter<T, C>
where
    T: Eq + Hash,
{
    /// Get a reference to the underlying HashMap
    pub fn get_map(&self) -> &AHashMap<T, C> {
        &self.map
    }

    /// Consume the counter and return the underlying HashMap
    pub fn into_map(self) -> AHashMap<T, C> {
        self.map
    }

    /// Calculate sum of all counts.
    pub fn total<'a, S>(&'a self) -> S
    where
        S: Sum<&'a C>,
    {
        self.map.values().sum()
    }
}

impl<T, C> Counter<T, C>
where
    T: Eq + Hash,
    C: Zero,
{
    pub fn new() -> Self {
        Self {
            map: AHashMap::new(),
            zero: C::zero(),
        }
    }
}

impl<T, C> Counter<T, C>
where
    T: Eq + Hash,
    C: AddAssign + Zero + One,
{
    pub fn init<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut counter = Self::new();
        counter.update(iterable);
        counter
    }

    pub fn update<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in iterable {
            let entry = self.map.entry(item).or_insert_with(C::zero);
            *entry += C::one();
        }
    }
}

impl<'a, T, C> IntoIterator for &'a Counter<T, C>
where
    T: Eq + Hash,
{
    type Item = (&'a T, &'a C);
    type IntoIter = Iter<'a, T, C>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}

impl<T, C> IntoIterator for Counter<T, C>
where
    T: Eq + Hash,
{
    type Item = (T, C);
    type IntoIter = IntoIter<T, C>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}
