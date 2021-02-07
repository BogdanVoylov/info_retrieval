use serde::{Deserialize, Serialize};

use std::{
    collections::{BTreeMap, HashSet},
    fs::File,
    str,
    string::String,
    time::SystemTime,
};

use crate::main::file_processor::{ChunkFileProcessor, FileProcessor};

use super::{single_file_index::*, string_utils::*, traits::*, *};

pub trait Extend {
    fn extend_from_file_processor<T: FileProcessor>(
        &mut self,
        sfc: SingleFileBiwordIndex<T>,
    ) -> usize;
}

pub struct Item {
    pub term: String,
    pub doc: String,
}

impl Extend for BTreeMap<String, HashSet<String>> {
    fn extend_from_file_processor<T: FileProcessor>(
        &mut self,
        sfc: SingleFileBiwordIndex<T>,
    ) -> usize {
        for term in sfc.data().iter() {
            let item = Item {
                term: term.clone(),
                doc: sfc.name(),
            };
            match self.get_mut(&item.term) {
                None => {
                    let mut set = HashSet::<String>::new();
                    set.insert(item.doc);
                    self.insert(item.term, set);
                }
                Some(set) => {
                    set.insert(item.doc);
                }
            }
        }
        return sfc.word_num();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultipleFileBiwordIndex {
    map: BTreeMap<String, HashSet<String>>,
    range: Vec<String>,
    collection_size: usize,
    name:String
}

impl MultipleFileIndex for MultipleFileBiwordIndex {
    fn process_concurrent(&mut self, input_names: &Vec<String>, buff_size: usize) {
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
                let mut sfc = SingleFileBiwordIndex::from_file_processor(
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
        println!(
            "Dict size {} | Collection size {} ",
            self.map.len(),
            self.collection_size
        );
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn serialize(&self) -> std::string::String {
        serde_json::to_string(self).unwrap()
    }
}

impl MultipleFileBiwordIndex {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            range: Vec::new(),
            collection_size: 0,
            name:"MultipleFileBiwordIndex".to_owned()
        }
    }

    fn parallel_join_to_map(
        &mut self,
        vec: &mut Vec<SingleFileBiwordIndex<ChunkFileProcessor>>,
    ) -> Vec<(BTreeMap<String, HashSet<String>>, usize)> {
        let mut handles: Vec<_> = Vec::new();
        let mut res_vec: Vec<_> = Vec::new();
        let chunk_size: usize = 2;
        let chunk_num: usize = vec.len() / chunk_size;
        for i in 0..(chunk_num + 1) {
            let mut v = vec![];
            for j in 0..chunk_size {
                if vec.len() == 0 {
                    break;
                }
                v.push(vec.remove(0))
            }
            handles.push(std::thread::spawn(move || {
                let mut collection_size = 0;
                let mut m = BTreeMap::<String, HashSet<String>>::new();
                for sfc in v {
                    collection_size += sfc.word_num();
                    m.extend_from_file_processor(sfc);
                }
                return (m, collection_size);
            }))
        }
        for h in handles {
            res_vec.push(h.join().unwrap());
            println!("extension joined");
        }
        res_vec
    }

    fn parallel_extend(&mut self, vec: &mut Vec<SingleFileBiwordIndex<ChunkFileProcessor>>) {
        let res_vec = self.parallel_join_to_map(vec);
        for (map, num) in res_vec {
            for (k, v) in map {
                match self.map.get_mut(&k) {
                    None => {
                        let mut set = HashSet::<String>::new();
                        self.map.insert(k, v);
                    }
                    Some(set) => {
                        set.extend(v);
                    }
                }
            }
            self.collection_size += num
        }
    }
}

impl PUT<Item> for MultipleFileBiwordIndex {
    fn put(&mut self, item: Item) {
        match self.map.get_mut(&item.term) {
            None => {
                let mut set = HashSet::<String>::new();
                set.insert(item.doc);
                self.map.insert(item.term, set);
            }
            Some(set) => {
                set.insert(item.doc);
            }
        }
    }
}

impl GET<&str,Vec::<(String, Vec<String>)>> for MultipleFileBiwordIndex {
    fn get(&self, idx: &str) -> Vec::<(String, Vec<String>)> {
        let idx_vec = StringUtils::replace_default(idx)
            .split_whitespace()
            .collect::<Vec<&str>>()
            .biword();
        let mut res_vec = Vec::<(String, Vec<String>)>::new();
        for idx in idx_vec {
            match self.map.get(&idx) {
                Some(r) => {
                    res_vec.push((idx.to_owned(), r.into_iter().map(|x| x.clone()).collect()))
                }
                None => {}
            }
        }
        res_vec
    }
}
