use super::super::file_processor::FileProcessor;

use std::{collections::{HashSet, HashMap}, str, string::String};

use super::string_utils::*;

pub trait SingleFileIndex <T: FileProcessor, V>{
    fn from_file_processor(file_processor: T) -> Self;
    fn word_num(&self)->usize;
    fn proccess(&mut self);
    fn name(&self) -> String;
    fn data(&self) -> &V;
}

pub struct SingleFileBiwordIndex<T: FileProcessor> {
    set: HashSet<String>,
    file_processor: T,
    word_num:usize,
}

impl <T:FileProcessor>SingleFileIndex<T, HashSet<String>> for SingleFileBiwordIndex<T> {
    fn from_file_processor(file_processor: T) -> Self {
        Self {
            set: HashSet::new(),
            file_processor,
            word_num:0
        }
    }

    fn word_num(&self)->usize{
        self.word_num
    }

    fn proccess(&mut self) {
        let buff = self.file_processor.process();
        let str_buff = str::from_utf8(buff.as_slice()).unwrap();
        let str_buff = StringUtils::replace_default(str_buff);
        let iter = str_buff.split_whitespace();
        let vec: Vec<&str> = iter.collect();
        self.word_num = vec.len();
        self.set = vec.biword();
    }

    fn data(&self) -> &HashSet<String>{
        &self.set
    }

    fn name(&self) -> String {
        self.file_processor.name()
    }
}


pub struct SingleFileCoorsIndex<T: FileProcessor> {
    map: HashMap<String,Vec<usize>>,
    file_processor: T,
    word_num:usize,
}

impl <T:FileProcessor>SingleFileIndex<T, HashMap<String,Vec<usize>>> for SingleFileCoorsIndex<T> {
    fn from_file_processor(file_processor: T) -> Self {
        Self {
            map: HashMap::new(),
            file_processor,
            word_num:0
        }
    }

    fn word_num(&self)->usize{
        self.word_num
    }

    fn proccess(&mut self) {
        let buff = self.file_processor.process();
        let str_buff = str::from_utf8(buff.as_slice()).unwrap();
        let str_buff = StringUtils::replace_default(str_buff);
        let iter = str_buff.split_whitespace();
        let vec: Vec<&str> = iter.collect();
        self.word_num = vec.len();
        for i in 0..vec.len() {
            let v = vec[i].to_owned();
            match self.map.get_mut(&v) {
                Some(r) => {r.push(i);}
                None => {
                    self.map.insert(v.to_owned(), vec![i]);
                }
            }
        }
    }

    fn data(&self) -> &HashMap<String,Vec<usize>>{
        &self.map
    }

    fn name(&self) -> String {
        self.file_processor.name()
    }
}
