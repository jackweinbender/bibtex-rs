use tag::Tag;

#[derive(Debug, PartialEq)]
pub struct EntryItem {
    key: String,
    pub_type: String,
    tags: Vec<Tag>,
}
impl EntryItem {}

#[derive(Debug, PartialEq)]
pub struct StringItem {
    key: String,
    content: String,
}
impl StringItem {
    pub fn new(s: &str) -> StringItem {
        let idx = s.find('=').unwrap();
        let k = &s[..idx];
        let c = &s[idx + 1..];
        StringItem {
            key: String::from(k.trim()),
            content: String::from(c.trim()),
        }
    }
}
#[test]
fn parse_string_item_from_str() {
    let input = StringItem::new("JLW = {Jack L. Weinbender}");
    assert_eq!(
        StringItem {
            key: String::from("JLW"),
            content: String::from("{Jack L. Weinbender}"),
        },
        input
    );
}


#[derive(Debug, PartialEq)]
pub struct PreambleItem(String);
impl PreambleItem {
    pub fn new(s: &str) -> PreambleItem {
        PreambleItem(String::from(s))
    }
}

#[test]
fn parse_preamble_from_str() {
    let input = "This is a Preamble";
    assert_eq!(
        PreambleItem(String::from("This is a Preamble")),
        PreambleItem::new(input)
    );
}

#[derive(Debug, PartialEq)]
pub struct CommentItem(String);

impl CommentItem {
    pub fn new(s: &str) -> CommentItem {
        CommentItem(String::from(s))
    }
}

#[test]
fn parse_comment_from_str() {
    let input = "This is a Comment";
    assert_eq!(
        CommentItem(String::from("This is a Comment")),
        CommentItem::new(input)
    );
}
