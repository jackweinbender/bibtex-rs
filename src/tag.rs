#[derive(Debug, PartialEq)]
pub struct Tag {
    key: String,
    content: String,
}

impl Tag {
    pub fn new(s: &str) -> Tag {
        let idx = s.find('=').unwrap();
        let k = &s[..idx];
        let c = &s[idx + 1..];
        Tag {
            key: String::from(k.trim().to_lowercase()),
            content: String::from(c.trim()),
        }
    }
}

#[test]
fn parse_tag_from_str() {
    let input = Tag::new("\tAuthor =  {Jack Weinbender}");
    assert_eq!(
        Tag {
            key: String::from("author"),
            content: String::from("{Jack Weinbender}"),
        },
        input
    );
}
