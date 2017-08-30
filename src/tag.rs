use std::str::FromStr;
use std::str::Utf8Error;

#[derive(Debug, PartialEq)]
struct Tag{key: String, content: String}

impl FromStr for Tag{
    type Err = Utf8Error;
    fn from_str(s: &str) -> Result<Tag, Utf8Error>{
        let idx = s.find('=').unwrap();
        let k = &s[..idx];
        let c = &s[idx +1..];
        Ok(Tag{
            key: String::from(k.trim().to_lowercase()),
            content: String::from(c.trim())})
    }
}

#[test]
fn parse_tag_from_str() {
    let input = "\tAuthor =  {Jack Weinbender}".parse::<Tag>();
    assert_eq!(Tag{key: String::from("author"), content: String::from("{Jack Weinbender}")}, input.unwrap());
}