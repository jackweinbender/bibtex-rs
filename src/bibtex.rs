// pub type BibtexBibliography = Vec<BibtexEntry>;

#[derive(Debug)]
pub enum Bibtex<'a> {
    Comment     {},
    Preamble    {},
    String      { key: &'a str, value: &'a str},
    Entry       { key: &'a str, t: &'a str, attrs: Vec<(&'a str, Vec<BibtexValue>)>}
}

#[derive(Debug)]
pub enum BibtexValue {
    BibtexString,
    String
}