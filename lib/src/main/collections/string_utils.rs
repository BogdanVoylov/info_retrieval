use std::{collections::HashSet, string::String, vec::Vec};
use unicode_segmentation::UnicodeSegmentation;
use rand::{distributions::Alphanumeric, Rng};

pub struct StringUtils {}

impl StringUtils {
    pub fn replace_default(s: &str) -> String {
        let s = s.replace(
            &[
                '(', ')', '-', ',', '\"', '.', ';', ':', '\'', '"', '?', '”', '“', '!', '/', '[',
                ']', '{', '}', '=', '&', '$', '#', '*'
            ][..],
            " ",
        )
        .to_lowercase();
        Self::make_ascii(s.as_str())
    }

    fn is_first_char_normal_ascii(c: &str) -> bool {
        c.as_bytes().get(0).map_or(false, |&c| c >= 32 && c <= 126)
    }

    pub fn make_ascii(s: &str) -> String {
        UnicodeSegmentation::graphemes(s, true)
            .map(|c| match c {
                c if c.len() == 1 && Self::is_first_char_normal_ascii(c) => c, // normal printable range
                "\t" => "\t",
                "\r" | "\n" | "\r\n" => "\n", // normalize all newlines
                "á" | "à" | "ã" => "a",
                "é" | "è" => "e",
                "í" | "ì" | "ĩ" => "i",
                "ó" | "ò" | "õ" => "o",
                "ú" | "ù" | "ũ" => "u",
                "ñ" | "ń" => "n",
                _ => {
                    // If the first character is a normal ascii character, then use it
                    if Self::is_first_char_normal_ascii(c) {
                        c.get(..1).unwrap_or("")
                    } else {
                        ""
                    }
                }
            })
            .collect()
    }

    pub fn random_string(len:usize) -> String  {
        rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
    }

    pub fn serialize_set(set:&HashSet<String>) -> String {
        let mut res = "[".to_owned();
        for v in set {
            res.push('\"');
            res.push_str(v);
            res.push('\"');
            res.push(',')
        }
        let len = res.len();
        if len > 1 {
            res.pop();
        }
        res.push(']');
        res
    }
}
