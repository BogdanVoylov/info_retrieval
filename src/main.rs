use lib::main::collection::{kgram_index::*, prefix_index::*, *, permuterm_index::*};

use std::io::{prelude::*, stdin, stdout, LineWriter};
use std::{env, fs, fs::File, time::SystemTime};

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    if args.contains(&"-p".to_owned()){
        process_prefix();
    } else if args.contains(&"-k".to_owned()){
        process_kgram();
    } else {
        process_permuterm();
    }

    /*  main::process_concurrent(&files, BUFF_SIZE) */
}

fn process_prefix() {
    let paths = fs::read_dir("./data").unwrap();
    let mut files = Vec::<String>::new();
    for path in paths {
        files.push(path.unwrap().path().to_str().unwrap().to_owned());
    }

    let mut index = PrefixIndex::new();

    index.process_concurrent(&files, 0);

    loop {
        let mut q = r_l("> ");
        q.truncate(q.len() - 1);
        let res = index.get(q);
        println!("{}", res.len());
        println!("{:?}", res);
    }
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
        println!("completed in {} ms",end-start);
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
        println!("completed in {} ms",end-start);
    }


}

fn save_index<T:MultipleFileIndex>(index:&T){
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
