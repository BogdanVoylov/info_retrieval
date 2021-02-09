use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Trie {
    is_word: bool,
    next: HashMap<char, Trie>,
    size: usize,
}

/**
* `&self` means the method takes an immutable reference.
* If you need a mutable reference, change it to `&mut self` instead.
*/
impl Trie {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Default::default()
    }

    /** Inserts a word into the trie. */
    pub fn insert(&mut self, word: String) {

        self.size += 1;
        let mut curr = self;
        for ch in word.chars() {
            curr = curr.next.entry(ch).or_insert(Trie::new());
        }
        curr.is_word = true;
    }

    /** Returns if the word is in the trie. */
    pub fn search(&self, word: String) -> bool {
        self.find(word).map_or(false, |t| t.is_word)
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    pub fn starts_with(&self, prefix: String) -> bool {
        self.find(prefix).is_some()
    }

    pub fn find(&self, word: String) -> Option<&Trie> {
        let mut curr = self;
        for ch in word.chars() {
            curr = curr.next.get(&ch)?;
        }
        Some(curr)
    }
}
/* ['m', 'i', 'o', '¡', 's', 'f', 'a', 'e', 'b', 'k', 'h', 'u', 'l'] */
pub struct TrieSearch {
    v: HashSet<String>,
}

impl TrieSearch {
    pub fn new() -> Self {
        Self { v: HashSet::new() }
    }
    pub fn find(&mut self, t: &Trie, word: &String) -> Vec<String> {
        let mut word = word.clone();
        match t.find(word.clone()) {
            Some(t) => {
                self.get_all_words(t, word);
            }
            None => {}
        };
        self.v.iter().map(|x|x.clone()).collect()
    }
    pub fn get_all_words(&mut self, t: &Trie, word: String) {
        if t.is_word {
            self.v.insert(word.clone());
        }
        for (ch, n) in t.next.iter() {
            let mut w = word.clone();
            w.push(ch.clone());
            self.get_all_words(&n, w);
        }
    }
}
