mod parse;
mod romaji;

fn main() {
    let res = parse::parse_hiragana_with_buf("じねんじょをたべるぞ", "zinennjix").unwrap();
    println!("{:?}", res)
}
