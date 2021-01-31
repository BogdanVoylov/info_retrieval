use super::super::file_processor::FileProcessor;

use std::{cell::RefCell, collections::{hash_set::Iter, HashSet}, str, string::String};

#[path = "./set.rs"]
mod set;

pub struct SingleFileCollection<T: FileProcessor> {
    set: HashSet<String>,
    file_processor: T,
}

impl<T: FileProcessor> SingleFileCollection<T> {
    pub fn from_file_processor(file_processor: T) -> Self {
        Self {
            set: HashSet::new(),
            file_processor,
        }
    }

    pub fn proccess(&mut self) {
        let set = HashSet::<String>::new();
        let set_cell = RefCell::new(set);
        self.file_processor.process(|buff| {
            let str_buff = str::from_utf8(buff.as_slice()).unwrap().to_owned();
            let str_buff = str_buff.replace(
                &[
                    '(', ')', ',', '\"', '.', ';', ':', '\'', '"', '?', '”', '“', '!', '/', '[', ']', '{', '}',
                ][..],
                " ",
            );
            let iter = str_buff.split_whitespace();
            let vec: Vec<&str> = iter.collect();
            if vec.len() != 0 {
                for i in 0..(vec.len() - 1) {
                    let part = vec[i];
                    set_cell.borrow_mut().insert(part.to_owned());
                }
            }
        });
        self.set = set_cell.into_inner();
    }

    pub fn iter(&self) -> Iter<'_, String>{
        self.set.iter()
    }

    pub fn name(&self) -> String {
        self.file_processor.name()
    }
}
