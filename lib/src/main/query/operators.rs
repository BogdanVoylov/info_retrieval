use crate::main::collection::GET;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "arg")]
pub enum Group {
    OPERATOR(Box<Operator>),
    WORD(String),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "op", content = "args")]
pub enum Operator {
    AND(Vec<Group>),
    OR(Vec<Group>),
    NOT(Vec<Group>),
}

pub trait ExecutableOperator {
    fn exec(&self, i: &Vec<Group>, val: &Box<dyn GET<String, Vec<String>>>) -> Vec<String>;
}

fn exec_g(
    g: &Group,
    val: &Box<dyn GET<String, Vec<String>>>,
) -> Vec<String> {
    match g {
        Group::WORD(w) => val.get(w),
        Group::OPERATOR(o) => exec(o, val),
    }
}

fn map_groups(
    vec: &Vec<Group>,
    val: &Box<dyn GET<String, Vec<String>>>,
) -> Vec<String> {
    vec.into_iter()
        .map(|g| exec_g(g, val))
        .flatten()
        .collect()
}

pub fn exec(operator: &Operator, val: &Box<dyn GET<String, Vec<String>>>) -> Vec<String> {
    match operator {
        Operator::NOT(args) => NotOperator::new().exec(args, val),
        Operator::AND(args) => AndOperator::new().exec(args, val),
        Operator::OR(args) => OrOperator::new().exec(args, val),
    }
}

struct AndOperator {}

impl AndOperator {
    pub fn new() -> Self {
        Self {}
    }
    fn each_has(&self, vecs: &Vec<Vec<String>>, val:&String) -> bool {
        for v in vecs {
            if !v.contains(val){
                return false;
            }
        }
        true
    }
    fn merge(&self, vecs: &mut Vec<Vec<String>>) -> Vec<String> {
        let mut v: Vec<String>  = vecs.iter().flatten().filter(|x| {
            self.each_has(vecs,x)
        }).map(|x|x.clone()).collect();
        v.sort();
        v.dedup();
        v
    }
}

impl ExecutableOperator for AndOperator {
    fn exec(&self, i: &Vec<Group>, val: &Box<dyn GET<String, Vec<String>>>) -> Vec<String> {
        /* let args = map_groups( &i, val); */
        let mut vecs: Vec<Vec<String>> = i
            .iter()
            .map(|v| {
                let mut res = match v {
                    Group::WORD(w) => val.get(w),
                    Group::OPERATOR(o) => exec(o,val)
                };
                res.sort();
                res
            })
            .collect();

        self.merge(&mut vecs)
    }
}

struct OrOperator {}

impl OrOperator {
    pub fn new() -> Self {
        Self {}
    }
}

impl ExecutableOperator for OrOperator {
    fn exec(&self, i: &Vec<Group>, val: &Box<dyn GET<String, Vec<String>>>) -> Vec<String> {
        let mut res: Vec<String> = i
        .iter()
        .map(|v| {
            let mut res = match v {
                Group::WORD(w) => val.get(w),
                Group::OPERATOR(o) => exec(o,val)
            };
            res.sort();
            res
        })
        .flatten()
        .collect();
        res.dedup();
        res
    }
}

struct NotOperator {}

impl NotOperator {
    pub fn new() -> Self {
        Self {}
    }

    fn compute(&self, vals: &Vec<String>, range:&Vec<String>) -> Vec<String>{
        let mut res = range.clone();
        for v in vals {
            res.retain(|x| x != v);
        }
        res
    }
}

impl ExecutableOperator for NotOperator {
    fn exec(&self, i: &Vec<Group>, val: &Box<dyn GET<String, Vec<String>>>) -> Vec<String> {
        let mut res: Vec<String> = i
            .iter()
            .map(|v| {
                match v {
                    Group::WORD(w) => val.get(w),
                    Group::OPERATOR(o) => exec(o,val)
                }
                
            })
            .flatten()
            .collect();
        res.dedup();
        self.compute(&res, &val.range())
    }
}
