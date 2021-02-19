use serde::{Deserialize, Serialize};

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::File,
    str,
    string::String,
    time::SystemTime,
};


use crate::main::file_processor::{ChunkFileProcessor, FileProcessor};

use super::{process_strategies::permuterm::*, single_file_index::*, string_utils::*, *};

type Item = HashMap<String, HashSet<String>>; // term -> set<docname>

#[derive(Serialize, Deserialize, Debug)]
pub struct MultipleFilePermutermIndex {
    map_left: BTreeMap<String, Item>,
    map_right: BTreeMap<String, Item>,
    collection_size: usize,
    name: String,
}

impl MultipleFileIndex for MultipleFilePermutermIndex {
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
            handles.push(std::thread::spawn(move || {
                let error_file_open_msg = Box::new(format!("Unable to open {}", &name));
                let file = File::open(name.clone()).expect(&*error_file_open_msg);

                let mut sfc = SingleFileProcessor::new();
                let fp = ChunkFileProcessor::from_file(file, name, buff_size);
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

        self.extend(res_vec);

        let after_merge = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        println!("merge in {} ms", after_merge - after_join);
        println!(
            "Dict size {} | Collection size {} ",
            self.map_left.len(),
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

impl MultipleFilePermutermIndex {
    pub fn new() -> Self {
        Self {
            map_left: BTreeMap::new(),
            map_right: BTreeMap::new(),
            collection_size: 0,
            name: "MultipleFilePermutermIndex".to_owned(),
        }
    }

    fn get_left(&self, s: &String) -> Option<&Item> {
        self.map_left.get(s)
    }

    fn get_right(&self, s: &String) -> Option<&Item> {
        self.map_right.get(s)
    }

    fn get_right_range(&self, s: String) -> Vec<&Item> {
        let mut end = s.clone();
        end.push('~');
        self.map_right.range(s..end).map(|(k, v)| v).collect()
    }

    pub fn get(&self, s: String) -> Item {
        let null = Item::new();
        if s == "*" {
            println!("Got invalid '*' query");
            return null;
        }
        let parts: Vec<String> = s.as_str().split("*").map(|x| x.to_owned()).collect();
        let mut base_item: Item = Item::new();
        let mut ranges = Vec::<HashSet<&String>>::new();

        match self.get_left(&parts[0]) {
            Some(i) => {
                base_item = i.clone();
                ranges.push(i.keys().collect())
            }
            None => {}
        };

        for i in 1..(parts.len() - 1) {
            // all but first and last
            let p = parts.get(i).unwrap().clone();
            if p.len() == 0 {
                break;
            }
            let i_range = self.get_right_range(p.clone());
            if i_range.len() == 0 {
                return null;
            }

            let mut buff = HashSet::<&String>::new();
            if i == 1 && base_item.len() == 0 {
                for item in i_range {
                    buff.extend(item.keys());
                    for (k, v) in item {
                        let docs = base_item.entry(k.clone()).or_insert(HashSet::new());
                        docs.extend(v.iter().map(|x| x.clone()));
                    }
                }
            } else {
                for item in i_range {
                    buff.extend(item.keys());
                }
            }

            if buff.len() > 0 {
                ranges.push(buff);
            }
        }

        let last = parts.last().unwrap();
        if last != "" {
            match self.get_right(last) {
                Some(v) => {
                    if base_item.len() == 0 {
                        base_item = v.clone();
                    }
                    let b = v.keys().collect();
                    ranges.push(b);
                }
                None => return null,
            }
        }

        let mut base = ranges.get(0).unwrap().clone();
        for set in ranges.iter().skip(1) {
            base = base.intersection(set).copied().collect();
        }

        let mut buff = Item::new();

        for term in base {
            let docs = base_item.get(term).unwrap();
            buff.insert(term.clone(), docs.iter().map(|x| x.clone()).collect());
        }

        buff
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn extend(&mut self, v: Vec<SingleFileProcessor>) {
        for sfp in v {
            self.collection_size += sfp.word_num();
            self.extend_from_sfp(&sfp)
        }
    }

    fn extend_from_sfp(&mut self, sfp: &SingleFileProcessor) {
        let doc = sfp.name();
        for w in sfp.data() {
            let p = Permuterm::process(w);
            for (l, r) in p {
                self.map_left.put(l, w.clone(), doc.clone());
                self.map_right.put(r, w.clone(), doc.clone());
            }
        }
    }
}

trait Put {
    fn put(&mut self, idx: String, term: String, doc: String);
}

impl Put for BTreeMap<String, Item> {
    fn put(&mut self, idx: String, term: String, doc: String) {
        if idx != "" {
            let e: &mut Item = self.entry(idx).or_insert(Item::new());
            let docs = e.entry(term.clone()).or_insert(HashSet::new());
            docs.insert(doc.clone());
        }
    }
}
