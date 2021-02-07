use std::{collections::HashSet, string::String, vec::Vec};

pub trait Biword {
    fn biword(&self) -> HashSet<String>;
}

impl Biword for Vec<&str> {
    fn biword(&self) -> HashSet<String> {
        StringUtils::biword(self)
    }
}

pub struct StringUtils {}

impl StringUtils {
    pub fn biword(vec: &Vec<&str>) -> HashSet<String> {
        let mut set = HashSet::<String>::new();
        for i in 0..vec.len() - 1 {
            let mut part = vec[i].to_owned();
            let s_part = vec.get(i + 1).unwrap_or(&"");
            part.push_str(&(" ".to_owned() + s_part));

            set.insert(part);
        }
        set
    }

    pub fn replace_default(s: &str) -> String {
        s.replace(
            &[
                '(', ')', '-', ',', '\"', '.', ';', ':', '\'', '"', '?', '”', '“', '!', '/', '[',
                ']', '{', '}',
            ][..],
            " ",
        ).to_lowercase()
    }
}
