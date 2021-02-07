use filebuffer::FileBuffer;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    string::String,
    vec::Vec,
};

pub trait FileProcessor {
    fn process(&self) -> Vec<u8>;
    fn name(&self) -> String;
}

pub struct ChunkFileProcessor {
    file: File,
    buff_size: usize,
    name: String,
}

impl ChunkFileProcessor {
    pub fn from_file(file: File, name: String, buff_size: usize) -> Self {
        Self {
            file,
            name,
            buff_size,
        }
    }
}

impl FileProcessor for ChunkFileProcessor {
    fn process(&self) -> Vec<u8> {
        FileBuffer::open(self.name()).unwrap().as_ref().to_owned()
    }

    fn name(&self) -> std::string::String {
        self.name.clone()
    }
}

fn get_end(buff: &mut Vec<u8>) -> Vec<u8> {
    let mut idx = buff.len() - 1;
    let res = Vec::<u8>::new();
    while idx != 0 && buff[idx] != (32 as u8) {
        idx -= 1;
    }
    return buff.split_off(idx);
}
