use serde::{Deserialize, Serialize};

use std::{
    collections::{BTreeMap, HashSet},
    fs::File,
    str,
    string::String,
    time::SystemTime,
};

use crate::main::file_processor::{ChunkFileProcessor};

use super::{single_file_index::*};

use super::prefix_trie::*;

#[derive(Default)]
pub struct PrefixIndex {
    trie:Trie
}

impl PrefixIndex {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn extend(&mut self, s:&SingleFileProcessor) {
        for w in s.data() {
            self.trie.insert(w.clone());
        }
    }

    pub fn extend_from_vec(&mut self, v:&Vec<SingleFileProcessor>) {
        for s in v {
            self.extend(s);
        }
    }

    pub fn get(&self,s:String) -> Vec<String> {
        let mut trie_search = TrieSearch::new();
        trie_search.find(&self.trie, &s)
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
            handles.push(std::thread::spawn(move || {
                let error_file_open_msg = Box::new(format!("Unable to open {}", &name));
                let file = File::open(name.clone()).expect(&*error_file_open_msg);

                let mut sfc = SingleFileProcessor::new();
                let mut fp = ChunkFileProcessor::from_file(file,name,buff_size);
                let fp = Box::new(fp);
                sfc.proccess(fp);

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

        self.extend_from_vec(&mut res_vec);

        let after_merge = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        println!("merge in {} ms", after_merge - after_join);
    }
}