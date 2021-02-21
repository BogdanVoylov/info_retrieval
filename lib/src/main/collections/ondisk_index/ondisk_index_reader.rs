use std::{
    cmp::Ordering,
    collections::*,
    fs::File,
    io::{BufRead, BufReader},
};

pub struct OndiskIndexReader {
    reader: BufReader<File>,
    empty: bool,
    key: String,
    value: String,
    name:String,
    parsed_value: HashSet<u32>
}

impl OndiskIndexReader {
    pub fn new(file_name: &String) -> Self {
        let reader: BufReader<File> = BufReader::new(File::open(file_name).unwrap());
        let mut res = Self {
            reader,
            empty: false,
            key: String::new(),
            value:String::new(),
            name:file_name.clone(),
            parsed_value: HashSet::<u32>::new()
        };
        res.read();
        res
    }

    pub fn rest(&mut self) -> Vec::<u8> {
        let mut buf = Vec::<u8>::new();
        self.reader.read_until(b'~', &mut buf);
        buf
    }

    fn read(&mut self) {
        let mut buf = String::new();
        self.reader.read_line(&mut buf);
        let key = buf.clone();
        let mut buf = String::new();
        match self.reader.read_line(&mut buf) {
            Ok(n) => {
                /* println!("buff {} {}", buf, n); */
                if n == 0 {
                    self.empty = true;
                    return;
                }
                self.key = key;
                self.value = buf;
                self.parsed_value = serde_json::from_str(self.value.as_str()).unwrap();
            }
            Err(e) => println!("{}", e),
        };
    }

    pub fn empty(&self) -> bool {
        self.empty
    }

    pub fn cmp(&self, s: &String) -> Ordering {
        self.key.cmp(s)
    }

    pub fn process(&mut self) -> bool {
        self.read();
        self.empty
    }

    pub fn value(&mut self) -> &String {
        &self.value
    }

    pub fn parsed_value(&self) -> HashSet<u32> {
        self.parsed_value.clone()
    }

    pub fn boxed_value(&self) -> Box<HashSet<u32>> {
        Box::new(self.parsed_value())
    }

    pub fn clone_key(&self) -> String {
        self.key.clone()
    }

    pub fn key(&self) -> &String {
        &self.key
    }
}
