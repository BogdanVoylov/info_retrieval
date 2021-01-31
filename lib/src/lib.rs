pub mod main {
    use std::str;

    use std::fs::File;

    use std::io::{prelude::*, BufRead, BufReader, LineWriter};

    use std::sync::{Arc, Mutex};

    use std::time::SystemTime;

    #[path = "./collections/multiple_file_collection.rs"]
    pub mod collection;

    pub mod file_processor;

    #[path = "./query/query.rs"]
    pub mod query;

    #[path = "./query/operators.rs"]
    pub mod operators;
    
   
}
