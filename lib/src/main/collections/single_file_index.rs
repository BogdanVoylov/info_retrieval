use super::super::file_processor::FileProcessor;

use std::{
    collections::{HashMap, HashSet},
    str,
    string::String,
};

use super::string_utils::*;

pub trait SingleFileIndex<V> {
    fn word_num(&self) -> usize;
    fn proccess(&mut self, fp: Box<dyn FileProcessor>);
    fn name(&self) -> String;
    fn data(&self) -> &V;
}

pub struct SingleFileProcessor {
    set: HashSet<String>,
    word_num: usize,
    name: String,
}

impl SingleFileProcessor {
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
            word_num: 0,
            name: String::new(),
        }
    }
}
use encoding::all::ASCII;
use encoding::{DecoderTrap, Encoding};
impl SingleFileIndex<HashSet<String>> for SingleFileProcessor {
    fn word_num(&self) -> usize {
        self.word_num
    }

    fn proccess(&mut self, fp: Box<dyn FileProcessor>) {
        self.name = fp.name();
        let buff = fp.process();
        /*  let str_buff = match str::from_utf8(buff.as_slice()) {
            Ok(v) => v,
            Err(e) => {println!("err {} in {}", e, self.name); return}
        }; */
        let str_buff = ASCII.decode(buff.as_slice(), DecoderTrap::Replace).unwrap();
        let str_buff = StringUtils::replace_default(str_buff.as_str());
        let iter = str_buff.split_whitespace();
        let vec: Vec<&str> = iter.collect();
        self.word_num = vec.len();
        for s in vec {
            self.set.insert(s.to_owned());
        }
    }

    fn data(&self) -> &HashSet<String> {
        &self.set
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
