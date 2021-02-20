use lib::main::collection::{kgram_index::*, permuterm_index::*, *};

use std::io::{prelude::*, stdin, stdout, LineWriter};
use std::{env, fs, fs::File, time::SystemTime, mem::*};

use lib::main::collection::ondisk_index::concurrent_ondisk_index::*;

const GB_SIZE:usize = 1073741824;

fn main() {
    let args: Vec<String> = env::args().collect();

    let v = read_dir_recur(Path::new("/mnt/store/gutenberg"));
    let v_len = v.len();

    let mut index = ConcurrentOndiskIndex::new(v);
    /* index.process_concurrent(); */
    index.reduce(read_dir_recur(Path::new("cache")));

    /*  main::process_concurrent(&files, BUFF_SIZE) */
}
use std::fs::*;
use std::path::Path;

fn read_dir_recur(dir: &Path) -> Vec<String> {
    let paths = fs::read_dir(dir).unwrap();
    let mut files = Vec::<String>::new();
    for v in paths {
        let path = v.unwrap().path();

        if path.is_dir() {
            files.append(&mut read_dir_recur(&path));
        } else {
            files.push(path.to_str().unwrap().to_owned());
        }
    }
    files
}

fn process_kgram() {
    let paths = fs::read_dir("./data").unwrap();
    let mut files = Vec::<String>::new();
    for path in paths {
        files.push(path.unwrap().path().to_str().unwrap().to_owned());
    }

    let mut index = MultipleFileKgramIndex::new();

    index.process_concurrent(&files, 0);

    save_index(&index);

    loop {
        let mut q = r_l("> ");
        q.truncate(q.len() - 1);
        let start = ms();
        let res = index.get(q);
        let res_str = serde_json::to_string_pretty(&res).unwrap();
        let end = ms();
        println!("{}", res_str);
        println!("result len: {}", res.len());
        println!("completed in {} ms", end - start);
    }
}

fn process_permuterm() {
    let paths = fs::read_dir("./data").unwrap();
    let mut files = Vec::<String>::new();
    for path in paths {
        files.push(path.unwrap().path().to_str().unwrap().to_owned());
    }

    let mut index = MultipleFilePermutermIndex::new();

    index.process_concurrent(&files, 0);

    save_index(&index);

    loop {
        let mut q = r_l("> ");
        q.truncate(q.len() - 1);
        let start = ms();
        let res = index.get(q);
        let res_str = serde_json::to_string_pretty(&res).unwrap();
        let end = ms();
        println!("{}", res_str);
        println!("result len: {}", res.len());
        println!("completed in {} ms", end - start);
    }
}

fn save_index<T: MultipleFileIndex>(index: &T) {
    let j = index.serialize();

    let file = File::create(index.name() + ".json").unwrap();
    let mut file = LineWriter::new(file);

    file.write_all(j.as_bytes());
    file.flush();
}

fn r_l(t: &str) -> String {
    print!("{}", t);
    stdout().flush();
    let mut buffer = String::new();
    let mut stdin = stdin();
    stdin.read_line(&mut buffer).unwrap();
    buffer
}

fn ms() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
