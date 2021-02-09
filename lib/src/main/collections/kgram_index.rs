use serde::{Deserialize, Serialize};

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    str,
    string::String,
    time::SystemTime,
};

use crate::main::file_processor::{ChunkFileProcessor, FileProcessor};

use super::{process_strategies::kgram::*, single_file_index::*, string_utils::*, *};

pub trait Extend {
    fn extend_from_file_processor(&mut self, sfc: SingleFileProcessor) -> usize;
}

type Item = HashMap<String, HashSet<String>>; // term -> set<docname>

impl Extend for HashMap<String, Item> {
    fn extend_from_file_processor(&mut self, sfc: SingleFileProcessor) -> usize {
        let doc = sfc.name();
        for term in sfc.data().iter() {
            let kgrams = Kgram::process(term.clone());
            for kgram in kgrams {
                let e: &mut Item = self.entry(kgram).or_insert(Item::new());
                let docs = e.entry(term.clone()).or_insert(HashSet::new());
                docs.insert(doc.clone());
            }
        }
        return sfc.word_num();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultipleFileKgramIndex {
    map: HashMap<String, Item>,
    collection_size: usize,
    name: String,
}

impl MultipleFileIndex for MultipleFileKgramIndex {
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
                let mut fp = ChunkFileProcessor::from_file(file, name, buff_size);
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

impl MultipleFileKgramIndex {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            collection_size: 0,
            name: "MultipleFileKgramIndex".to_owned(),
        }
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn get(&self, s:String) -> Item {
        let null = Item::new();
        let mut base_item = Item::new();
        let kgrams = s.as_str().split("*");
        let mut ranges = Vec::<HashSet<&String>>::new();
        for kgram in kgrams {
            if kgram.len() == 0 {
                break;
            }
            match self.map.get(kgram) {
                Some(v) => {
                    if base_item.len() == 0{
                        base_item = v.clone();
                    }
                    ranges.push(v.keys().collect());
                }
                None => return null
            }
        }
        let mut base = ranges.get(0).unwrap().clone();
        for set in ranges.iter().skip(1) {
            base = base.intersection(set).copied().collect();
        }

        let mut res = Item::new();
        for term in base {
            let docs = base_item.get(term).unwrap();
            res.insert(term.clone(), docs.iter().map(|x| x.clone()).collect());
        }
        res
    }

    fn extend(&mut self, v: Vec<SingleFileProcessor>) {
        for sfp in v {
            self.collection_size+=sfp.word_num();
            self.map.extend_from_file_processor(sfp);
        }
    }
}
