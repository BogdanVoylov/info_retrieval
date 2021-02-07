use lib::main::collection::{biword_index::*, coors_index::*, traits::GET, *};

use std::io::{prelude::*, stdin, stdout, LineWriter};
use std::{env, fs, fs::File, time::SystemTime};

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let need_update = args.contains(&"-p".to_owned());
    let enable_coors = args.contains(&"-c".to_owned());

    let mut index: Box<dyn MultipleFileIndex> = if enable_coors {
        Box::new(MultipleFileCoorsIndex::new())
    } else {
        Box::new(MultipleFileBiwordIndex::new())
    };
    if need_update {
        proccess_files(&mut index);
    }

    if enable_coors {
        exec_query_coors("MultipleFileCoorsIndex.json");
    } else {
        exec_query_biword("MultipleFileBiwordIndex.json")
    }

    /*  main::process_concurrent(&files, BUFF_SIZE) */
}

fn proccess_files(index: &mut Box<dyn MultipleFileIndex>) {
    let paths = fs::read_dir("./data").unwrap();
    let mut files = Vec::<String>::new();
    for path in paths {
        files.push(path.unwrap().path().to_str().unwrap().to_owned());
    }
    const KB_SIZE: usize = 1024;

    const MB_SIZE: usize = KB_SIZE * KB_SIZE;

    const BUFF_SIZE: usize = 1 * MB_SIZE;

    index.process_concurrent(&files, BUFF_SIZE);

    let after_merge = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let j = index.serialize();

    let after_serialized = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    println!("serialized in {} ms", after_serialized - after_merge);

    let file = File::create(index.name() + ".json").unwrap();
    let mut file = LineWriter::new(file);

    file.write_all(j.as_bytes());
    file.flush();
}

fn exec_query_biword(name: &str) {
    let collection_str = std::fs::read_to_string(name).unwrap();
    let data = std::fs::read_to_string("query.json").unwrap();

    let collection_parsing_start = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let collection: MultipleFileBiwordIndex =
        serde_json::from_str(collection_str.as_str()).unwrap();

    let collection_parsing_end = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    println!(
        "parsed collection in {} ms",
        collection_parsing_end - collection_parsing_start
    );

    loop {
        let query = r_l("> ");
        let res = collection.get(query.as_str());
        println!("------ QUERY RESULT ------");
        for r in res {
            let (k, v) = r;
            println!("{} {:?}", k, v);
        }
        let completed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        println!("completed in {} ms", completed - collection_parsing_end);
        println!("------ !QUERY RESULT ------")
    }
}

fn exec_query_coors(name: &str) {
    let collection_str = std::fs::read_to_string(name).unwrap();
    let data = std::fs::read_to_string("query.json").unwrap();

    let collection_parsing_start = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let collection: MultipleFileCoorsIndex = serde_json::from_str(collection_str.as_str()).unwrap();

    let collection_parsing_end = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    println!(
        "parsed collection in {} ms",
        collection_parsing_end - collection_parsing_start
    );

    loop {
        let query_str = r_l("> ");
        let res = collection.get(query_str.as_str());
        println!("------ QUERY RESULT ------");
        println!("{}", res);
        let completed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        println!("completed in {} ms", completed - collection_parsing_end);
        println!("------ !QUERY RESULT ------")
    }
}

fn r_l(t: &str) -> String {
    print!("{}", t);
    stdout().flush();
    let mut buffer = String::new();
    let mut stdin = stdin();
    stdin.read_line(&mut buffer).unwrap();
    buffer
}
