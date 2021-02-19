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
}

impl OndiskIndexReader {
    pub fn new(file_name: String) -> Self {
        let reader: BufReader<File> = BufReader::new(File::open(file_name).unwrap());
        let mut res = Self {
            reader,
            empty: false,
            key: String::new(),
            value:String::new(),
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
            }
            Err(e) => println!("{}", e),
        };
    }

    pub fn readable(&self) -> bool {
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

    pub fn parsed_value(&self) -> HashSet<String> {
        serde_json::from_str(self.value.as_str()).unwrap()
    }

    pub fn clone_key(&self) -> String {
        self.key.clone()
    }

    pub fn key(&self) -> &String {
        &self.key
    }
}
