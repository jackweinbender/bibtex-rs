// src/bibtex.rs

#[derive(Debug,PartialEq)]
pub enum Bibtex<'a> {
    Comment {},
    Preamble {},
    String {
        key: &'a str, value: &'a str
    },
    Entry {   
        key: &'a str, 
        t: &'a str, 
        attrs: Vec<(&'a str, Vec <AttrValue<'a> >)>
    }
}

#[derive(Debug,PartialEq)]
pub enum AttrValue<'a> {
    String(&'a str),
    Number(u32),
    Alias(&'a str)
}