use std::{collections::{HashMap, hash_map::Iter}, string::String};
use std::iter::{FromIterator, FusedIterator};
pub struct Set {
    map: HashMap<String, usize>,
}

impl Set {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn put(&mut self, term: String) {
        match self.map.get(&term) {
            None => {
                self.map.insert(term, 1);
            }
            Some(count) => {
                self.map.insert(term, count + 1);
            }
        }
    }

    pub fn iter(&self) -> Iter<'_, String, usize>{
        self.map.iter()
    }
}

