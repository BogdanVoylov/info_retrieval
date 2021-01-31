use serde::{Deserialize, Serialize};

use std::{
    collections::{BTreeMap, HashSet},
    fs::{File},
    str,
    string::String,
    time::SystemTime,
};


mod single_file_collection;

use super::file_processor::{ChunkFileProcessor, FileProcessor};

pub trait CRUD<Idx, Input, Output> : GET<Idx,Output> {
    /* type Idx; */
    fn put(&mut self, item: Input);
}

pub trait GET<Idx, Output> {
    fn get(&self, idx:&Idx) -> Output;
    fn range(&self) -> Output;
}

pub struct Item {
    pub term: String,
    pub doc: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultipleFileCollection {
    map: BTreeMap<String, HashSet<String>>,
    range: Vec<String>
}

impl MultipleFileCollection {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            range: Vec::new()
        }
    }

    fn extend<T: FileProcessor>(&mut self, sfc: single_file_collection::SingleFileCollection<T>) {
        for term in sfc.iter() {
            self.put(Item {
                term: term.clone(),
                doc: sfc.name(),
            })
        }
    }

    fn parallel_extend(&mut self, vec:&mut Vec<single_file_collection::SingleFileCollection<ChunkFileProcessor>>){
        let mut handles: Vec<_> = Vec::new();
        let mut res_vec: Vec<_> = Vec::new();
        let chunk_size:usize = 3;
        let chunk_num:usize = vec.len()/chunk_size;
        for i in 0..(chunk_num+1){
            let mut v = vec![];
            for j in 0..chunk_size{
                let idx = i+j;
                if idx >= vec.len() {
                    break;
                }
                v.push(vec.remove(idx))
            }
            handles.push(std::thread::spawn(move || {
                let mut m = MultipleFileCollection::new();
                for sfc in v {
                    m.extend(sfc);
                }
                return m;
            }))
        }
        for h in handles {
            let map = h.join().unwrap();
            res_vec.push(map);
            println!("extension joined");
        }
        for el in res_vec{
            self.map.extend(el.map);
        }
        
    }

    pub fn process_concurrent(&mut self, input_names: &Vec<String>, buff_size: usize) {
        let mut res_vec: Vec<_> = Vec::new();
        let begin = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let input_names = input_names.clone();
        let mut handles: Vec<_> = Vec::new();
        for name in input_names.into_iter() {
            let name = name.to_owned();
            self.range.push(name.clone());
            handles.push(std::thread::spawn(move || {
                let error_file_open_msg = Box::new(format!("Unable to open {}", &name));
                let file = File::open(name.clone()).expect(&*error_file_open_msg);
                let mut sfc = single_file_collection::SingleFileCollection::from_file_processor(
                    ChunkFileProcessor::from_file(file, name.clone(), buff_size),
                );
                sfc.proccess();
                return sfc;
            }));
        }
        for h in handles {
            let sfc = h.join().unwrap();
            res_vec.push(sfc);
            println!("joined");
        }

        let after_join = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        println!("joined all in {} ms", after_join - begin);

        self.parallel_extend(&mut res_vec);

        let after_merge = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        println!("merge in {} ms", after_merge - after_join);
    }

   
}

impl CRUD<String, Item, Vec<String>> for MultipleFileCollection {
    fn put(&mut self, item: Item) {
        match self.map.get_mut(&item.term) {
            None => {
                let mut set = HashSet::<String>::new();
                set.insert(item.doc);
                self.map.insert(item.term, set);
            }
            Some(set) => {set.insert(item.doc);}
        }
    }

    
}

impl GET<String, Vec<String>> for MultipleFileCollection {
    fn get(&self, idx:&String) -> Vec<String> {
        let res = self.map.get(idx);
        res.iter()
        .map(|v| v.iter())
        .flatten()
        .map(|v| v.clone())
        .collect()
    }

    fn range(&self) -> Vec<String> {
        self.range.clone()
    }
}
