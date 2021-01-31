use lib::main::collection::MultipleFileCollection;
use lib::main::query::QueryTree;
use lib::main::collection::GET;

use std::{env,fs,time::SystemTime,fs::{File}};
use std::io::{prelude::*, LineWriter};


fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    if !args.contains(&"-p".to_owned())  {
        proccess_files();
    }

   
    exec_query();
    

    /*  main::process_concurrent(&files, BUFF_SIZE) */
}

fn proccess_files() {

    let paths = fs::read_dir("./data").unwrap();
    let mut files = Vec::<String>::new();
    for path in paths {
        files.push(path.unwrap().path().to_str().unwrap().to_owned());
    }
    const KB_SIZE: usize = 1024;

    const MB_SIZE: usize = KB_SIZE * KB_SIZE;

    const BUFF_SIZE: usize = 8 * KB_SIZE;
    

    let mut collection = MultipleFileCollection::new();

    collection.process_concurrent(&files, BUFF_SIZE);

    let after_merge = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();

    let j = serde_json::to_string(&collection).unwrap();

    let after_serialized = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();

    println!("serialized in {} ms", after_serialized-after_merge);

    let file = File::create("res.json").unwrap();
    let mut file = LineWriter::new(file);

    file.write_all(j.as_bytes());
    file.flush();
}

fn exec_query(){
    let collection_str = std::fs::read_to_string("res.json").unwrap();
    let data = std::fs::read_to_string("query.json").unwrap();


    let q = QueryTree::parse(data.to_owned());

    let collection_parsing_start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();

    let collection: MultipleFileCollection = serde_json::from_str(collection_str.as_str()).unwrap();

    let collection_parsing_end = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();

    println!("parsed collection in {} ms", collection_parsing_end - collection_parsing_start);

    let get:Box<dyn GET<String,Vec<String>>> = Box::new(collection);

    let res = q.exec(&get);

    println!("------ QUERY RESULT ------");
    for s in res {
        println!("{}",s);
    }
    let completed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    println!("completed in {} ms", completed - collection_parsing_end);
    println!("------ !QUERY RESULT ------")
}
