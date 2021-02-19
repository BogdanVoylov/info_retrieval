use std::{
    time::SystemTime,
    fs::File
};

use crate::main::file_processor::{ChunkFileProcessor};
use super::single_file_index::*;


pub fn process_files_concurrent(input_names: &Vec<String>, buff_size:usize) -> Vec<SingleFileProcessor> {
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
    res_vec
}