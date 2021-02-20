use super::{ondisk_index::*, ondisk_index_reducer::*};
use std::collections::*;

use crate::main::collection::string_utils::*;

type Collection = HashMap<String, usize>;

pub struct ConcurrentOndiskIndex {
    c_vec: Vec<String>,
    collection: Collection,
}

const buff_size: usize = 13687090;
impl ConcurrentOndiskIndex {
    pub fn new(c: Vec<String>) -> Self {
        let mut collection = Collection::new();
        for i in 0..c.len() {
            collection.insert(c[i].clone(), i);
        }
        Self {
            c_vec: c,
            collection,
        }
    }

    pub fn process_concurrent(&mut self) {
        let mut handles: Vec<_> = Vec::new();
        for (i, chunk) in self
            .collection
            .iter()
            .collect::<Vec<_>>()
            .chunks(self.collection.len() / 10)
            .enumerate()
        {
            let mut c = Collection::new();
            for (k, v) in chunk {
                c.insert(k.clone().clone(), v.clone().clone());
            }
            handles.push(std::thread::spawn(move || {
                let name = format!("{}_{}", i, StringUtils::random_string(5));
                let mut idx = OndiskIndex::new(name, c, buff_size);
                idx.proccess();
            }));
        }

        for h in handles {
            h.join().unwrap();
        }
    }

    pub fn reduce(&mut self, filenames: Vec<String>) {
        OndiskIndexReducer::help_reduce(filenames, format!("index/{}",StringUtils::random_string(5)))
    }
}

/*  OndiskIndexReducer::help_reduce(filenames, format!("index/{}",StringUtils::random_string(5))) */
/* 
let mut handles: Vec<_> = Vec::new();
        for (i, c) in filenames.chunks(2).enumerate() {
            if c.len() < 2 {
                break;
            }
            let i1 = c[0].clone();
            let i2 = c[1].clone();
            handles.push(std::thread::spawn(move || {
                OndiskIndexReducer::help_reduce_p(
                    i1,
                    i2,
                    format!("index/{}_{}", i, StringUtils::random_string(5)),
                )
            }));
        }

        for h in handles {
            h.join().unwrap();
        }
 */