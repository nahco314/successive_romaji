use crate::romaji::BASIC_ROMAJI_CHARS;
use pyo3::pyclass;
use std::fmt::Debug;

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WritingChar {
    pub hiragana: String,
    pub romaji: String,
    pub cur_buf_string: String,
}

impl WritingChar {
    pub fn new(hiragana: &str, romaji: &str, cur_buf_string: &str) -> WritingChar {
        WritingChar {
            hiragana: hiragana.to_string(),
            romaji: romaji.to_string(),
            cur_buf_string: cur_buf_string.to_string(),
        }
    }
}

pub fn try_read<'a, 'b>(
    hiragana: &'a str,
    romaji: &'b str,
) -> Option<(&'a str, &'b str, (String, String))> {
    if hiragana.is_empty() {
        return None;
    }

    for (h, r) in BASIC_ROMAJI_CHARS {
        if hiragana.starts_with(h) && romaji.starts_with(r) {
            return Some((
                &hiragana[h.len()..],
                &romaji[r.len()..],
                (h.to_string(), r.to_string()),
            ));
        }
    }

    let mut chars = romaji.chars();
    let first_char = chars.next();
    let second_char = chars.next();

    if hiragana.starts_with('っ') && romaji.len() >= 2 && first_char == second_char {
        // if C is 'n', it will hit rule of "nn -> ん"
        // if C is vowel, it will hit simple rule of vowel (for example, "a -> あ")
        // so we don't need to check it here
        // where C is the first (and also second) character of romaji

        return Some((
            &hiragana[3..],
            &romaji[1..],
            ("っ".to_string(), romaji[..1].to_string()),
        ));
    }

    if hiragana.starts_with('ん') && romaji.len() >= 2 && first_char.unwrap() == 'n' {
        // if chars[1] is 'n', it will hit rule of "nn -> ん"
        // if chars[1] is vowel, it will hit rule of ん行 (for example, "na -> な")
        // so we don't need to check it here

        return Some((
            &hiragana[3..],
            &romaji[1..],
            ("ん".to_string(), "n".to_string()),
        ));
    }

    None
}

fn find_non_sokuonn(hiragana: &str) -> char {
    if hiragana.is_empty() {
        panic!("empty hiragana");
    }

    for c in hiragana.chars() {
        if c != 'っ' {
            return c;
        }
    }

    // if all characters are 'っ'
    return 'っ';
}

fn get_writing<'a>(hiragana: &'a str, rest: &str) -> Option<(&'a str, WritingChar)> {
    for (h, r) in BASIC_ROMAJI_CHARS {
        if hiragana.starts_with(h) && r.starts_with(rest) {
            return Some((
                &hiragana[h.len()..],
                WritingChar {
                    hiragana: h.to_string(),
                    romaji: r.to_string(),
                    cur_buf_string: rest.to_string(),
                },
            ));
        }
    }

    if hiragana.starts_with('っ') && rest.len() == 1 {
        let first_char = rest;
        let non_sokuonn = find_non_sokuonn(hiragana);

        for (h, r) in BASIC_ROMAJI_CHARS {
            if non_sokuonn.to_string() == h && r.starts_with(rest) {
                return Some((
                    &hiragana[3..],
                    WritingChar {
                        hiragana: 'っ'.to_string(),
                        romaji: first_char.to_string(),
                        cur_buf_string: first_char.to_string(),
                    },
                ));
            }
        }
    }

    None
}

fn get_basic_rule_of_char(c: char) -> String {
    for (h, r) in BASIC_ROMAJI_CHARS {
        if h.starts_with(c) {
            return r.to_string();
        }
    }

    panic!("invalid romaji: {}", c)
}

fn parse_one_check_one_n(hiragana: &str) -> Option<(&str, (String, String))> {
    if hiragana.starts_with('ん') && hiragana.chars().count() >= 2 {
        let second_char = hiragana.chars().nth(1).unwrap();

        let mut ok = true;

        for c in "あいうえおん".chars() {
            if c == second_char {
                ok = false;
                break;
            }
        }

        if ok {
            return Some((&hiragana[3..], ("ん".to_string(), "n".to_string())));
        }
    }

    None
}

fn try_parse_one(hiragana: &str) -> Option<(&str, (String, String))> {
    if hiragana.starts_with('っ') && hiragana.chars().count() > 1 {
        let non_sokuonn = find_non_sokuonn(hiragana);
        let first_char = get_basic_rule_of_char(non_sokuonn);
        return Some((
            &hiragana[3..],
            ('っ'.to_string(), first_char[..1].to_string()),
        ));
    }

    if let Some(val) = parse_one_check_one_n(hiragana) {
        return Some(val);
    }

    if hiragana.starts_with('ん') && hiragana.chars().count() > 2 {
        let non_sokuonn = find_non_sokuonn(hiragana);
        let first_char = get_basic_rule_of_char(non_sokuonn);
        return Some((
            &hiragana[3..],
            ('っ'.to_string(), first_char[..1].to_string()),
        ));
    }

    for (h, r) in BASIC_ROMAJI_CHARS {
        if hiragana.starts_with(h) {
            return Some((&hiragana[h.len()..], (h.to_string(), r.to_string())));
        }
    }

    None
}

pub fn parse_hiragana(mut hiragana: &str) -> Option<Vec<(String, String)>> {
    let mut res = vec![];
    while !hiragana.is_empty() {
        let part;
        (hiragana, part) = try_parse_one(hiragana)?;
        res.push(part);
    }

    return Some(res);
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParseResult {
    Writing(Vec<(String, String)>, WritingChar, Vec<(String, String)>),
    Completed(Vec<(String, String)>),
}

pub fn parse_hiragana_with_buf(mut hiragana: &str, mut romaji: &str) -> Option<ParseResult> {
    let mut confirmed = vec![];

    while let Some(val) = try_read(hiragana, romaji) {
        let part;
        (hiragana, romaji, part) = val;
        confirmed.push(part);
    }

    if hiragana.is_empty() {
        return Some(ParseResult::Completed(confirmed));
    }

    let writing;
    (hiragana, writing) = get_writing(hiragana, romaji)?;

    let tail = parse_hiragana(hiragana)?;

    return Some(ParseResult::Writing(confirmed, writing, tail));
}
