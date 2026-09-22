use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

use indexmap::IndexMap;

pub trait MapGet<K, V> {
    fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;
}

impl<K, V> MapGet<K, V> for HashMap<K, V>
where
    K: Hash + Eq,
{
    fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        HashMap::get(self, k)
    }
}

impl<K, V> MapGet<K, V> for IndexMap<K, V>
where
    K: Hash + Eq,
{
    fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        IndexMap::get(self, k)
    }
}
