pub struct OndiskIndexReducer {}
use std::{
    collections::*,
    fs::File,
    io::{prelude::*, LineWriter},
    time::SystemTime,
};
use std::ptr;
use super::{ondisk_index::*, ondisk_index_reader::*};
use crate::main::collection::string_utils::*;

impl OndiskIndexReducer {
    pub fn help_reduce(i_names: Vec<String>, res_name: String) {
        let mut v: Vec<OndiskIndexReader> = i_names.iter().map(|x|{OndiskIndexReader::new(x)}).collect();
        let o = File::create("res/k").unwrap();
        let mut k_o = LineWriter::new(o);
        let o = File::create("res/v").unwrap();
        let mut v_o = LineWriter::new(o);

        while v.len() > 0 {
            let mut k = v[0].key();
            let mut idxs = vec![0];
            let mut vls = v[0].parsed_value();
            for i in 1..v.len() {
                let item = &v[i];
                if item.key() < k {
                    k = item.key();
                    idxs = vec![i];
                    vls = item.parsed_value();
                } else if item.key() == k {
                    idxs.push(i);
                    vls.extend(item.parsed_value());
                }
            }

            /* println!("k {}", k);
 */

            k_o.write_all(k.as_bytes());
            let mut vl = serde_json::to_string(&vls).unwrap();
            vl.push('\n');
            v_o.write_all(vl.as_bytes());

            for i in idxs {
                let item = &mut v[i];
                item.process();
                if item.empty() {
                    println!("empty");
                }
            }
            v.retain(|x|!x.empty())
        }
    }

    pub fn help_reduce_p(i1: String, i2: String, res_name: String) {
        println!("\n ----- res: {} -----",res_name);
        let o_f = File::create(res_name.clone()).unwrap();
        let mut o = LineWriter::new(o_f);
        let mut i1 = OndiskIndexReader::new(&i1);
        let mut i2 = OndiskIndexReader::new(&i2);

        let i1_eof = false;
        let i2_eof = false;

        while !i1.empty() || !i2.empty() {
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

        if i1.empty() {
            o.write_all(i2.rest().as_slice());
        } else if i2.empty() {
            o.write_all(i1.rest().as_slice());
        }
    }

}

