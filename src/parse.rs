use crate::romaji::ROMAJI_CHARS;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WritingChar {
    hiragana: String,
    romaji: String,
    cur_buf_string: String,
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

fn try_read<'a, 'b>(
    hiragana: &'a str,
    romaji: &'b str,
) -> Option<(&'a str, &'b str, (String, String))> {
    for (h, r) in ROMAJI_CHARS {
        if hiragana.starts_with(h) && romaji.starts_with(r) {
            return Some((
                &hiragana[h.len()..],
                &romaji[r.len()..],
                (h.to_string(), r.to_string()),
            ));
        }
    }

    None
}

fn get_writing<'a>(hiragana: &'a str, rest: &str) -> Option<(&'a str, WritingChar)> {
    for (h, r) in ROMAJI_CHARS {
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

    None
}

fn try_parse_one(hiragana: &str) -> Option<(&str, (String, String))> {
    for (h, r) in ROMAJI_CHARS {
        if hiragana.starts_with(h) {
            return Some((&hiragana[h.len()..], (h.to_string(), r.to_string())));
        }
    }

    None
}

fn parse_hiragana(mut hiragana: &str) -> Option<Vec<(String, String)>> {
    let mut res = vec![];
    while !hiragana.is_empty() {
        let part;
        (hiragana, part) = try_parse_one(hiragana)?;
        res.push(part);
    }

    return Some(res);
}

pub fn parse_hiragana_with_buf(
    mut hiragana: &str,
    mut romaji: &str,
) -> Option<(Vec<(String, String)>, WritingChar, Vec<(String, String)>)> {
    let mut confirmed = vec![];

    while let Some(val) = try_read(hiragana, romaji) {
        let part;
        (hiragana, romaji, part) = val;
        confirmed.push(part);
    }

    let writing;
    (hiragana, writing) = get_writing(hiragana, romaji)?;

    let tail = parse_hiragana(hiragana)?;

    return Some((confirmed, writing, tail));
}
