
#[macro_use]
extern crate log;

use std::cmp::Ordering;

#[derive(Debug, Eq)]
struct Field<'a> (&'a str, &'a str);

impl<'a> Ord for Field<'a>  {
    fn cmp(&self, other: &Field) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<'a> PartialOrd for Field<'a>  {
    fn partial_cmp(&self, other: &Field) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Field<'a> {
    fn eq(&self, other: &Field) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug)]
struct BibEntry<'a> {
    entry_type: &'a str,
    cite_key: &'a str,
    fields: Vec<Field<'a>>,
}

impl<'a> BibEntry<'a> {
    pub fn cite_key(&self) -> &str {
        &self.cite_key
    }
    pub fn entry_type(&self) -> &str {
        &self.entry_type
    }
    pub fn field(&self, field: &str) -> &str {
        let fields = &self.fields;
        let index = fields.iter().position(|x| 
            x.0 == field ).expect("No key found.");
        fields[index].1
    }
    pub fn to_string(&mut self) -> String {
        &self.sort();
        let mut entry = String::new();

        entry.push_str("@");
        entry.push_str(&self.entry_type.to_string());
        entry.push_str("{");
        entry.push_str(&self.cite_key.to_string());
        
        if *&self.fields.is_empty() {
            entry.push_str("}}");
            return entry
        }

        for field in &self.fields {
            entry.push_str(",\n  ");
            entry.push_str(field.0);
            entry.push_str(" = {");
            entry.push_str(field.1);
            entry.push_str("}");
        }

        entry.push_str("\n}");
        entry
        
    }
    fn sort(&mut self) {
        &self.fields.sort();
    }
}

mod test{
    use super::*;

    #[test]
    fn getters() {
        let bib = include_str!("../test/single_sorted.bib");
        let mut v = Vec::new();
        v.push(Field("title", "The Historiographical {Jesus}: Memory, Typology, and the Son of {David}"));
        v.push(Field("shorttitle", "The Historiographical Jesus"));
        v.push(Field("author", "Le Donne, Anthony"));
        v.push(Field("location", "Waco, TX"));
        v.push(Field("publisher", "Baylor University Press"));
        v.push(Field("year", "2009"));

        let mut e = BibEntry {
            entry_type: "book",
            cite_key: "le-donne2009",
            fields: v,
        };

        assert_eq!(e.cite_key(), "le-donne2009");
        assert_eq!(e.entry_type(), "book");
        assert_eq!(e.field("author"), "Le Donne, Anthony");
        assert_eq!(e.to_string(), bib);
    }
}