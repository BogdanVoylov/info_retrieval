use std::{
    collections::*, 
    fs::{File},
    io::{
        LineWriter,
        prelude::*
    }

};

use crate::main::{
    collection::{single_file_index::*, string_utils::*},
    file_processor::*,
};

pub struct OndiskIndex {
    map: BTreeMap<String, HashSet<usize>>,
    size:usize,
    name: String,
    aliases: HashMap<String,usize>,
    size_limit: usize,
    delta: usize,
}

impl OndiskIndex {
    pub fn new(name: String, aliases:HashMap<String,usize>, size_limit: usize) -> Self {
        Self {
            map: BTreeMap::new(),
            size:0,
            name,
            aliases,
            size_limit,
            delta: 10000,
        }
    }

    pub fn proccess(&mut self,) -> Vec<String> {
        println!("started processing {} {}", self.name, self.size_limit, );
        let mut res_vec = Vec::<String>::new();
        for (i, (name, alias)) in self.aliases.clone().iter().enumerate() {
            println!("processing file {} {} {}", name, self.map.len(), self.size);
            if self.size_limit < self.delta + self.size  {
                println!("extended file limit");
                let name = format!(
                    "{}_{}_{}",
                    self.name,
                    i,
                    StringUtils::random_string(7)
                );
                self.cache(name.clone());
                res_vec.push(name);
            }
            let file = File::open(name.clone())
                .expect(format!("error opening file {} ", name.clone()).as_str());

            let mut sfp = SingleFileProcessor::new();
            let fp = ChunkFileProcessor::from_file(file, name.clone(), 8000);
            sfp.proccess(Box::new(fp));
            self.extend_from_sfp(sfp);
        }

        let name = format!("{}_{}", self.name, StringUtils::random_string(7));

        self.cache(name.clone());
        res_vec.push(name);
        res_vec
    }

    fn extend_from_sfp(&mut self, sfp: SingleFileProcessor) {
        let f_name = self.aliases.get(&sfp.name()).unwrap();
        self.size += sfp.data().len();
        for word in sfp.data() {
            let set = self.map.entry(word.clone()).or_insert(HashSet::new());
            set.insert(f_name.clone());
        }
    }

    fn cache(&mut self, name: String) {
        println!("caching size {}",self.size);
        let mut file = File::create(format!("cache/{}", name)).unwrap();
        let mut output = LineWriter::new(file);
        for (k, v) in &self.map {
            let v = serde_json::to_string(v).unwrap();
            output.write_all(format!("{}\n",k).as_bytes());
            output.write_all(format!("{}\n",v).as_bytes());
        }
        self.map.clear();
        self.size = 0;
        println!("after cache {}",self.map.len());
    }
}
