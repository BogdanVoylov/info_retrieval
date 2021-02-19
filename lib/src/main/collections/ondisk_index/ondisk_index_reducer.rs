pub struct OndiskIndexReducer {}
use std::{
    collections::*,
    fs::File,
    io::{prelude::*, LineWriter},
    time::SystemTime,
};

use super::{ondisk_index::*, ondisk_index_reader::*};
use crate::main::collection::string_utils::*;

impl OndiskIndexReducer {
    pub fn help_reduce(i1: String, i2: String, res_name: String) {
        println!("\n ----- res: {} -----",res_name);
        let o_f = File::create(res_name.clone()).unwrap();
        let mut o = LineWriter::new(o_f);
        let mut i1 = OndiskIndexReader::new(i1);
        let mut i2 = OndiskIndexReader::new(i2);

        let i1_eof = false;
        let i2_eof = false;

        while !i1.readable() || !i2.readable() {
            let i1_k = i1.key();
            let i2_k = i2.key();
            // println!("names:{} res_name:{} k1:{} k2:{}", names, res_name, i1_k, i2_k);
            if i1_k < i2_k {
                o.write_all(i1_k.as_bytes());
                o.write_all(i1.value().as_bytes());
                i1.process();
            } else if i1_k == i2_k {
                let mut value = i1.parsed_value();
                value.extend(i2.parsed_value());
                let value = serde_json::to_string(&value).unwrap();
                o.write_all(i1_k.as_bytes());
                o.write_all(value.as_bytes());
                i1.process();
                i2.process();
            } else {
                o.write_all(i2_k.as_bytes());
                o.write_all(i2.value().as_bytes());
                i2.process();
            }

        };

        println!("almost completed");

        if i1.readable() {
            o.write_all(i1.rest().as_slice());
        } else if i2.readable() {
            o.write_all(i2.rest().as_slice());
        }
    }
}
