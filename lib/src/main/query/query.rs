use serde::{Deserialize, Serialize};
use crate::main::{collection::GET, operators, operators::{Operator}};


/* NOT(A) */


#[derive(Serialize, Deserialize)]
pub struct QueryTree {
    query:Operator   
}

impl QueryTree {
    pub fn parse(str: String) -> Self {
       serde_json::from_str(str.as_str()).unwrap()
    }

    pub fn exec(&self, val:&Box<dyn GET<String, Vec<String>>>) -> Vec<String>{
        operators::exec(&self.query, val)
    }
}