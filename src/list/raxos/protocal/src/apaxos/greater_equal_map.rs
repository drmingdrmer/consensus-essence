use std::cmp::Ordering;
use std::fmt;

use crate::apaxos::greater_equal::GreaterEqual;

/// A map that stores values with [`GreaterEqual`] keys.
///
/// It provides the API to get all **maximal** keys.
/// A key is **maximal** if no other key is greater than it.
/// See: https://en.wikipedia.org/wiki/Partially_ordered_set#Extrema
#[derive(Clone)]
pub struct Map<K, V> {
    inner: Vec<(K, V)>,
}

impl<K, V> fmt::Debug for Map<K, V>
where
    K: fmt::Debug + GreaterEqual,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sorted_kvs: Vec<(&K, &V)> = self.inner.iter().map(|(k, v)| (k, v)).collect();

        sorted_kvs.sort_unstable_by(|(k1, _), (k2, _)| {
            if GreaterEqual::greater_equal(*k1, *k2) {
                Ordering::Greater
            } else if GreaterEqual::greater_equal(*k2, *k1) {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        f.debug_map().entries(sorted_kvs).finish()
    }
}

impl<K, V> Map<K, V>
where K: GreaterEqual
{
    /// Returns true if the given key will be an **maximal** if it is inserted.
    ///
    /// A key is **maximal** if no other key is greater than it.
    pub fn is_maximal(&self, key: &K) -> bool {
        self.inner.iter().all(|(k, _v)| {
            if k.greater_equal(key) {
                // if a >= b and b >= a, it is a maximal key.
                key.greater_equal(k)
            } else {
                true
            }
        })
    }

    /// Return key value pairs of all **maximal** keys.
    ///
    /// A key is **maximal** if no other key is greater than it.
    /// See: https://en.wikipedia.org/wiki/Partially_ordered_set#Extrema
    pub fn maximals(&self) -> impl Iterator<Item = (&K, &V)> {
        self.inner.iter().filter(move |(k1, _)| self.is_maximal(k1)).map(|(k, v)| (k, v))
    }
}

impl<K, V> Map<K, V>
where K: GreaterEqual + Eq
{
    pub fn new() -> Self {
        Map { inner: Vec::new() }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        for (k, v) in self.inner.iter_mut() {
            if *k == key {
                return Some(std::mem::replace(v, value));
            }
        }
        self.inner.push((key, value));
        None
    }

    pub fn try_insert(&mut self, key: K, value: V) -> Result<&mut V, ()> {
        for (k, _v) in self.inner.iter_mut() {
            if *k == key {
                return Err(());
            }
        }
        self.inner.push((key, value));
        Ok(&mut self.inner.last_mut().unwrap().1)
    }

    // TODO: test it
    pub fn remove(&mut self, key: &K) -> Option<V> {
        // 1. find the index of the specified key
        // 2. swap_remove it

        let index = self.inner.iter().position(|(k, _)| k == key)?;
        Some(self.inner.swap_remove(index).1)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.inner.iter().position(|(k, _)| k == key)?;
        Some(&self.inner[index].1)
    }

    // TODO: test
    pub fn merge(&mut self, other: Self) {
        for (k, v) in other.inner.into_iter() {
            // TODO: assert that `self` does not contain `k`, or contains equal `v`
            self.insert(k, v);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::apaxos::greater_equal::GreaterEqual;
    use crate::apaxos::greater_equal_map::Map;

    /// Define a greater-or-equal relation for P by modulo:
    /// 2 >= 1;
    /// 3 >= 1;
    /// 6 >= 2; 6 >= 3
    /// 0 >= 6; 0 >= 3; 0 >= 2; 0 >= 1
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct P(u64);

    impl GreaterEqual for P {
        fn greater_equal(&self, other: &Self) -> bool {
            self.0 % other.0 == 0
        }
    }

    #[test]
    fn test_map_maximals() {
        fn get_maximals(map: &Map<P, u64>) -> Vec<u64> {
            let maximals = map.maximals().map(|(k, _v)| k.0).collect::<BTreeSet<u64>>();
            maximals.into_iter().collect()
        }

        let mut map = Map::new();

        map.insert(P(2), 2);
        map.insert(P(3), 3);
        map.insert(P(6), 6);
        assert_eq!(get_maximals(&map), [6]);

        map.insert(P(9), 9);
        assert_eq!(get_maximals(&map), [6, 9]);
    }

    #[test]
    fn test_map_is_maximal() {
        let mut map = Map::new();

        map.insert(P(2), 2);
        map.insert(P(3), 3);
        map.insert(P(6), 6);
        assert!(map.is_maximal(&P(6)));
        assert!(map.is_maximal(&P(9)));
        assert!(map.is_maximal(&P(12)));
        assert!(!map.is_maximal(&P(3)));
        assert!(!map.is_maximal(&P(2)));
        assert!(!map.is_maximal(&P(1)));
    }

    #[test]
    fn test_map_debug() {
        let mut map = Map::new();

        map.insert(P(1), 1);
        map.insert(P(2), 2);
        map.insert(P(5), 5);
        map.insert(P(10), 10);
        map.insert(P(3), 3);
        map.insert(P(6), 6);
        map.insert(P(9), 9);

        let got = format!("{:?}", map);
        println!("{}", got);
    }
}
