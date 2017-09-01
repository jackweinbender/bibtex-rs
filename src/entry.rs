use tag::Tag;

#[derive(Debug, PartialEq)]
pub struct EntryItem {
    key: String,
    pub_type: String,
    tags: Vec<Tag>,
}
impl EntryItem {
    pub fn new(s: &str) -> EntryItem {
        let mut t = String::new();
        let mut k = String::new();
        let mut tags = vec![];
        let mut iter = s.char_indices();

        while let Some((idx, ch)) = iter.next() {
            match ch {
                '@' if idx == 0 => continue,
                '{' => break,
                _ => t.push(ch),
            }
        }
        while let Some((_, ch)) = iter.next() {
            match ch {
                '}' => break,
                ',' => {
                    if let Some((i, _)) = iter.next() {
                        let ts = s[i..].trim().trim_right_matches(',');

                        tags = ts[..ts.len() - 1]
                            .split("\n")
                            .filter(|x| x.trim() != "")
                            .map(|x| Tag::new(x.trim_right_matches(',')))
                            .collect();
                    };
                    break;
                }
                _ => k.push(ch),
            }
        }
        EntryItem {
            pub_type: t,
            key: k,
            tags: tags,
        }
    }
}

#[test]
fn parse_entry_item_from_str() {
    let input = include_str!("../test/single_unsorted.bib");
    let expected = EntryItem {
        key: String::from("le-donne2009"),
        pub_type: String::from("book"),
        tags: vec![
            Tag::new("title = {The Historiographical {Jesus}}"),
            Tag::new("author = {Le Donne, Anthony}"),
        ],
    };
    assert_eq!(EntryItem::new(input), expected);
}


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
