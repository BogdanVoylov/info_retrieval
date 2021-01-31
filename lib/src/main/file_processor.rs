use std::{
    fs::File,
    io::{ BufRead, BufReader },
    string::String,
};

pub trait FileProcessor {
    fn process<F: Fn(std::vec::Vec<u8>)>(&self, process: F);
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
    fn process<F: Fn(std::vec::Vec<u8>)>(&self, process: F) {
        let input_file = &self.file;
        let mut reader = BufReader::with_capacity(self.buff_size, input_file);
        let mut ending = Vec::<u8>::new();
        loop {
            let buffer: &[u8] = reader.fill_buf().unwrap();
            let mut buff_vec = ending.clone();
            if buffer.len() == 0 {
                process(buff_vec);
                break;
            }
            buff_vec.extend_from_slice(buffer);
            ending = get_end(&mut buff_vec);
            process(buff_vec);
            /* writer.write_all(&buffer).unwrap(); */
            reader.consume(self.buff_size);
        }
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
