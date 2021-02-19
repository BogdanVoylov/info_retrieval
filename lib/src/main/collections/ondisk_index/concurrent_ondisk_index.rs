use super::ondisk_index::*;
use std::collections::*;

use crate::main::collection::string_utils::*;

type Collection = HashMap<String,String>;

pub struct ConcurrentOndiskIndex {
    collection: Collection,
}

const buff_size: usize = 536870912;
impl ConcurrentOndiskIndex {
    pub fn new(c: Vec<String>) -> Self {
        let mut collection = Collection::new();
        for i in 0..c.len() {
            collection.insert(c[i],i.to_string());
        }
        Self { collection }
    }

    pub fn process_concurrent(&mut self) {

        let mut handles: Vec<_> = Vec::new();
        for (i,chunk) in self.collection.iter().collect::<Vec<_>>().chunks(self.collection.len() / 10).enumerate() {
            let mut c = Collection::new();
            for (k,v) in chunk {
                c.insert(k.clone().clone(),v.clone().clone());
            }
            handles.push(std::thread::spawn(move || {
                let name  = format!("{}_{}",i, StringUtils::random_string(5));
                let mut idx = OndiskIndex::new(name,c,buff_size);
                idx.proccess();
            }));
        }

        for h in handles {
            h.join().unwrap();
        }
    }

    pub fn reduce(&mut self, filenames: Vec<String>) {}
}
