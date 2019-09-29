// TODO

// testing ground
// DO NOT USE RIGHT NOW!

use std::collections::{hash_map::Keys, HashMap};
use std::hash::Hash;
use std::ops::{Index, IndexMut};
use std::slice;

#[derive(Default)]
pub struct HashVector<T: Eq + Hash, U> {
    map: HashMap<T, usize>,
    vec: Vec<U>,
}

impl<T: Eq + Hash + Clone, U> HashVector<T, U> {
    pub fn insert(&mut self, key: T, value: U) {
        debug_assert!(!self.map.contains_key(&key));

        self.map.insert(key, self.vec.len());
        self.vec.push(value);
    }

    pub fn remove(&mut self, key: &T) {
        // get index from map
        if let Some(index) = self.map.remove(key) {
            // check for out of bounds
            debug_assert!(index < self.vec.len());

            self.vec.remove(index);

            self.reorder(index);
        }
    }

    fn reorder(&mut self, index: usize) {
        // removes the last element
        let last_index = self.vec.len() - 1;

        if last_index != index {
            let last_value = self.vec.remove(last_index);

            // inserts the last element to the given location
            self.vec.insert(index, last_value);
        }

        // update map value entry
        for value in self.map.values_mut() {
            if *value == last_index + 1 {
                *value = index;
            }
        }
    }

    pub fn keys(&self) -> Keys<T, usize> {
        self.map.keys()
    }

    pub fn iter(&self) -> slice::Iter<U> {
        self.vec.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<U> {
        self.vec.iter_mut()
    }
}

impl<T: Eq + Hash + Clone, U> HashVector<T, U> {
    pub fn remove_at(&mut self, index: usize) {
        // check for out of bounds
        debug_assert!(index < self.vec.len());

        // remove vector element at index
        self.vec.remove(index);

        // search for all keys with index as value and remove them
        let mut keys = Vec::new();

        for (key, value) in self.map.iter() {
            if *value == index {
                keys.push(key.clone());
            }
        }

        for key in keys {
            self.map.remove(&key);
        }

        self.reorder(index);
    }
}

impl<'a, T: Eq + Hash, U> IntoIterator for &'a HashVector<T, U> {
    type Item = &'a U;
    type IntoIter = slice::Iter<'a, U>;

    fn into_iter(self) -> slice::Iter<'a, U> {
        self.vec.iter()
    }
}

impl<'a, T: Eq + Hash, U> IntoIterator for &'a mut HashVector<T, U> {
    type Item = &'a mut U;
    type IntoIter = slice::IterMut<'a, U>;

    fn into_iter(self) -> slice::IterMut<'a, U> {
        self.vec.iter_mut()
    }
}

// access via index directly to the vector
impl<T: Eq + Hash, U> Index<usize> for HashVector<T, U> {
    type Output = U;

    fn index(&self, index: usize) -> &U {
        &self.vec[index]
    }
}

impl<T: Eq + Hash, U> IndexMut<usize> for HashVector<T, U> {
    fn index_mut(&mut self, index: usize) -> &mut U {
        &mut self.vec[index]
    }
}

// access via key through the map to the vector
impl<'a, T: Eq + Hash, U> Index<&'a T> for HashVector<T, U> {
    type Output = U;

    fn index(&self, key: &'a T) -> &U {
        &self.vec[self.map[key]]
    }
}

impl<'a, T: Eq + Hash, U> IndexMut<&'a T> for HashVector<T, U> {
    fn index_mut(&mut self, key: &'a T) -> &mut U {
        &mut self.vec[self.map[key]]
    }
}

#[test]
fn hash_vec_insert_iterate() {
    let mut hash_vec: HashVector<&str, &str> = HashVector::default();

    let keys: Vec<&str> = vec!["62", "234", "test", "bla"];

    hash_vec.insert(keys[0], "frank");
    hash_vec.insert(keys[1], "sdf");
    hash_vec.insert(keys[2], "qwrrgf");
    hash_vec.insert(keys[3], "egh");

    for (index, item) in hash_vec.iter().enumerate() {
        assert_eq!(hash_vec[&keys[index]], *item);
        assert_eq!(hash_vec[index], *item);
    }
}

#[test]
fn hash_vec_delete_order() {
    for i in 0..4 {
        let mut keys: Vec<&str> = vec!["62", "234", "test", "bla"];

        let mut hash_vec: HashVector<&str, &str> = HashVector::default();

        hash_vec.insert(keys[0], "frank");
        hash_vec.insert(keys[1], "sdf");
        hash_vec.insert(keys[2], "qwrrgf");
        hash_vec.insert(keys[3], "egh");

        hash_vec.remove(&keys[i]);
        keys.remove(i);

        for (key, index) in hash_vec.map.iter() {
            let k = *index;
            assert_eq!(hash_vec[key], hash_vec[k]);
        }
    }
}
