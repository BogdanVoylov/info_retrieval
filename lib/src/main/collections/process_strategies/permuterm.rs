pub struct Permuterm;
impl Permuterm {
    pub fn new() -> Self {
        Self
    }

    pub fn process(s: &String) -> Vec<(String,String)> {
        let mut res = Vec::<(String,String)>::new();
        for i in 0..(s.len()+1){
            let (l,r) = s.split_at(i);
            res.push((l.to_owned(),r.to_owned()));
        }
        res
    }
}
