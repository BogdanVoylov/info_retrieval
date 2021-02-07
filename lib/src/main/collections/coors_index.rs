use serde::{Deserialize, Serialize};

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::File,
    str,
    string::String,
    time::SystemTime,
};

use crate::main::file_processor::{ChunkFileProcessor, FileProcessor};

use super::{single_file_index::*, string_utils::*, traits::*, *};

trait Extend {
    fn extend_from_file_processor<T: FileProcessor>(
        &mut self,
        sfc: SingleFileCoorsIndex<T>,
    ) -> usize;
}

type Item = HashMap<String, Vec<usize>>;

impl Extend for BTreeMap<String, Item> {
    fn extend_from_file_processor<T: FileProcessor>(
        &mut self,
        sfi: SingleFileCoorsIndex<T>,
    ) -> usize {
        let doc = sfi.name();
        for (term, poses) in sfi.data().iter() {
            let doc = sfi.name();
            let term = term.clone();
            match self.get_mut(&term) {
                None => {
                    let mut map = Item::new();
                    map.insert(doc, poses.clone());
                    self.insert(term, map);
                }
                Some(map) => {
                    map.insert(doc, poses.clone());
                }
            }
        }
        return sfi.word_num();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultipleFileCoorsIndex {
    map: BTreeMap<String, HashMap<String, Vec<usize>>>,
    range: Vec<String>,
    collection_size: usize,
    name:String
}

impl MultipleFileIndex for MultipleFileCoorsIndex {
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
                let mut sfi = SingleFileCoorsIndex::from_file_processor(
                    ChunkFileProcessor::from_file(file, name.clone(), buff_size),
                );
                sfi.proccess();
                return sfi;
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

impl MultipleFileCoorsIndex {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            range: Vec::new(),
            collection_size: 0,
            name:"MultipleFileCoorsIndex".to_owned()
        }
    }

    fn parallel_join_to_map(
        &mut self,
        vec: &mut Vec<SingleFileCoorsIndex<ChunkFileProcessor>>,
    ) -> Vec<(BTreeMap<String, Item>, usize)> {
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
                let len = v.len();
                let mut collection_size = 0;
                let mut m = BTreeMap::<String, HashMap<String, Vec<usize>>>::new();
                for sfi in v {
                    collection_size += sfi.word_num();
                    m.extend_from_file_processor(sfi);
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

    fn parallel_extend(&mut self, vec: &mut Vec<SingleFileCoorsIndex<ChunkFileProcessor>>) {
        let res_vec = self.parallel_join_to_map(vec);
        for (map, num) in res_vec {
            for (k, v) in map {
                match self.map.get_mut(&k) {
                    None => {
                        self.map.insert(k, v);
                    }
                    Some(map) => {
                        map.extend(v);
                    }
                }
            }
            self.collection_size += num
        }
    }
}
type uvec = Vec<usize>;
fn map_vec(v1: &uvec, v2: &uvec, dist: usize) -> uvec {
    let mut res: uvec = vec![];
    for e1 in v1 {
        for e2 in v2 {
            if e2 >= e1 && &(e1 + dist) >= e2 {
                res.push(e2.clone());
            }
        }
    }
    res
}

fn map_item(i1: &Item, i2: &Item, dist: usize) -> Option<Item> {
    let mut res_item = Item::new();
    for (k, vec) in i1 {
        match i2.get(k) {
            Some(vec2) => {
                let map_res = map_vec(vec, vec2, dist);
                if map_res.len() > 0 {
                    res_item.insert(k.clone(), map_res);
                }
            }
            None => {}
        };
    }

    if res_item.len() == 0 {
        None
    } else {
        Some(res_item)
    }
}

fn parse_query(q:&str) -> (Vec<&str>, Vec<usize>) {
    let v:Vec<&str> = q.split_whitespace().collect();
    let mut str_vec = Vec::<&str>::new();
    let mut u_vec = Vec::<usize>::new();
    for idx in v.iter().step_by(2) {
        str_vec.push(idx);
    }
    for idx in v.iter().skip(1).step_by(2) {
        u_vec.push(idx.parse().unwrap());
    }
    (str_vec, u_vec)
}
/*Disapointing 3 first 1 spotted 2 this 2 when 2 was*/

impl GET<&str, String> for MultipleFileCoorsIndex {
    fn get(&self, q: &str) -> String {
        let none_s: String = "None".to_owned();

        let (idx_vec, mut dist_vec)  = parse_query(q);

        let idx = idx_vec[0];
        let idx = StringUtils::replace_default(idx);
        let mut res_map: Item = match self.map.get(idx.as_str()) {
            Some(v) => v.clone(),
            None => return none_s,
        };

        for i in 1..idx_vec.len() {
            let idx = idx_vec[i];
            println!("-- iter {} --",i);
            match self.map.get(idx) {
                Some(r) => {
                    res_map = match map_item(&res_map, r, dist_vec.remove(0)) {
                        Some(r) => r,
                        None => return none_s,
                    }
                }
                None => {
                    break;
                }
            }
        }

        serde_json::to_string(&res_map).unwrap()
    }
}
