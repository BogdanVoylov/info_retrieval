pub struct Kgram {}
impl Kgram {
    
    pub fn process(s: String) -> Vec<String> {
        let s = format!("${}$", s);
        let mut res = Vec::<String>::new();
        for i in 3..s.len()+1 {
            let kgram = s.clone().drain((i - 3)..i).collect();
            res.push(kgram);
        }
        res
    }
}
