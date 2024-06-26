mod parse;
#[cfg(feature = "pyo3")]
#[cfg(not(test))]
mod py_wrap;
mod romaji;

pub use parse::*;
#[cfg(feature = "pyo3")]
#[cfg(not(test))]
pub use py_wrap::*;

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
        let res = parse::parse_hiragana_with_buf("じねんじょをたべるぞ", "jinennjix").unwrap();
        assert_eq!(
            res,
            ParseResult::Writing(
                tr(&[("じ", "ji"), ("ね", "ne"), ("ん", "nn"), ("じ", "ji")]),
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
    fn test_parse_hiragana_with_buf_n() {
        let res = parse::parse_hiragana_with_buf("じねんじょをたべるぞ", "jinen").unwrap();
        assert_eq!(
            res,
            ParseResult::Writing(
                tr(&[("じ", "ji"), ("ね", "ne")]),
                WritingChar::new("ん", "n", "n"),
                tr(&[
                    ("じょ", "jyo"),
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
        let res = parse::parse_hiragana_with_buf("じねんじょをたべるぞ", "jinennjixs");
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

    #[test]
    fn test_parse_hiragana_n() {
        let res = parse::parse_hiragana("あんき");

        assert_eq!(res, Some(tr(&[("あ", "a"), ("ん", "n"), ("き", "ki")])))
    }

    #[test]
    fn test_syo_or_sho() {
        let res = parse::parse_hiragana("しょ");

        assert_eq!(res, Some(tr(&[("しょ", "syo")])))
    }

    #[test]
    fn test_tyo_or_chyo() {
        let res = parse::parse_hiragana("ちょ");

        assert_eq!(res, Some(tr(&[("ちょ", "tyo")])))
    }

    #[test]
    fn test_writing_sokuon() {
        let res = parse::parse_hiragana_with_buf("ぺっと", "pe");

        assert_eq!(
            res,
            Some(ParseResult::Writing(
                tr(&[("ぺ", "pe")]),
                WritingChar::new("っ", "t", ""),
                tr(&[("と", "to")]),
            ))
        )
    }

    #[test]
    fn test_writing_n() {
        let res = parse::parse_hiragana_with_buf("かんか", "ka");

        assert_eq!(
            res,
            Some(ParseResult::Writing(
                tr(&[("か", "ka")]),
                WritingChar::new("ん", "n", ""),
                tr(&[("か", "ka")]),
            ))
        )
    }

    #[test]
    fn test_parse_n_0() {
        let res = parse::parse_hiragana("あんにん");

        assert_eq!(
            res,
            Some(tr(&[("あ", "a"), ("ん", "nn"), ("に", "ni"), ("ん", "nn")]))
        )
    }

    #[test]
    fn test_parse_n_1() {
        let res = parse::parse_hiragana("あんよ");

        assert_eq!(res, Some(tr(&[("あ", "a"), ("ん", "nn"), ("よ", "yo")])))
    }
}
