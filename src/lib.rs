
#[macro_use] extern crate log;

use std::collections::HashMap;

struct BibEntry<'a> {
    entry_type: &'a str,
    cite_key: &'a str,
    fields: HashMap<&'a str, &'a str>
}

impl<'a> BibEntry<'a> {
    pub fn cite_key(&self) -> &str {
        &self.cite_key
    }
    pub fn entry_type(&self) -> &str {
        &self.entry_type
    }
    pub fn field(&self, field: &str) -> &str {
        &self.fields.get(field).expect("No field found")
    }
}

#[cfg(test)]
mod tests {
    use super::BibEntry;
    use super::HashMap;

    #[test]
    fn getters(){

        let mut h = HashMap::new();
        h.insert("author", "Jack Weinbender");
        h.insert("title", "Remembering Rewritten Bible");
        h.insert("year", "2018");

        let e = BibEntry{
            entry_type: "thesis",
            cite_key: "weinbender2018",
            fields: h,
        };

        assert!(e.cite_key() == "weinbender2018");
        assert!(e.entry_type() == "thesis");
        assert!(e.field("author") == "Jack Weinbender");
        assert!(e.field("title") == "Remembering Rewritten Bible");
        assert!(e.field("year") == "2018");
    }
}
