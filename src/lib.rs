mod parse;
mod romaji;

pub use parse::*;

#[cfg(test)]
mod tests {
    use crate::{parse, ParseResult, WritingChar};

    fn tr(v: &[(&str, &str)]) -> Vec<(String, String)> {
        return v
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();
    }

    #[test]
    fn test_parse_hiragana_with_buf() {
        let res = parse::parse_hiragana_with_buf("じねんじょをたべるぞ", "zinennjix").unwrap();
        assert_eq!(
            res,
            ParseResult::Writing(
                tr(&[("じ", "zi"), ("ね", "ne"), ("ん", "nn"), ("じ", "ji")]),
                WritingChar::new("ょ", "xyo", "x"),
                tr(&[
                    ("を", "wo"),
                    ("た", "ta"),
                    ("べ", "be"),
                    ("る", "ru"),
                    ("ぞ", "zo")
                ]),
            )
        );
    }

    #[test]
    fn test_parse_hiragana_with_buf_error() {
        let res = parse::parse_hiragana_with_buf("じねんじょをたべるぞ", "zinennjixs");
        assert_eq!(res, None);
    }

    #[test]
    fn test_parse_hiragana_with_buf_done() {
        let res =
            parse::parse_hiragana_with_buf("じねんじょをたべるぞ", "jinennjyowotaberuzo").unwrap();

        assert_eq!(
            res,
            ParseResult::Completed(tr(&[
                ("じ", "ji"),
                ("ね", "ne"),
                ("ん", "nn"),
                ("じょ", "jyo"),
                ("を", "wo"),
                ("た", "ta"),
                ("べ", "be"),
                ("る", "ru"),
                ("ぞ", "zo")
            ]))
        )
    }

    #[test]
    fn test_parse_continual_sokuonn() {
        let res = parse::parse_hiragana("っっっっか");

        assert_eq!(
            res,
            Some(tr(&[
                ("っ", "k"),
                ("っ", "k"),
                ("っ", "k"),
                ("っ", "k"),
                ("か", "ka")
            ]))
        )
    }

    #[test]
    fn test_parse_only_sokuonn() {
        let res = parse::parse_hiragana("っっっっ");

        assert_eq!(
            res,
            Some(tr(&[("っ", "l"), ("っ", "l"), ("っ", "l"), ("っ", "ltu"),]))
        )
    }

    #[test]
    fn test_parse_hiragana_with_buf_only_sokuonn() {
        let res = parse::parse_hiragana_with_buf("かっっっか", "kakk").unwrap();

        assert_eq!(
            res,
            ParseResult::Writing(
                tr(&[("か", "ka"), ("っ", "k")]),
                WritingChar {
                    hiragana: "っ".to_string(),
                    romaji: "k".to_string(),
                    cur_buf_string: "k".to_string()
                },
                tr(&[("っ", "k"), ("か", "ka")])
            )
        )
    }

    #[test]
    fn test_parse_hiragana_with_buf_only_sokuonn_done() {
        let res = parse::parse_hiragana_with_buf("かっっっか", "kakkkka").unwrap();

        assert_eq!(
            res,
            ParseResult::Completed(tr(&[
                ("か", "ka"),
                ("っ", "k"),
                ("っ", "k"),
                ("っ", "k"),
                ("か", "ka")
            ]),)
        )
    }
}
